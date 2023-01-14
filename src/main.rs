#![forbid(unsafe_code)]
#![deny(clippy::all)]
mod keymap;
mod logs;

use clap::{arg, command, Parser};
use keymap::Keymap;
use pixels::{Pixels, SurfaceTexture};

use tao::{
    dpi::LogicalSize,
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    menu::{MenuBar, MenuItem},
    window::{Window, WindowBuilder},
};

use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use chip8::{
    cpu::{
        cpu::{CPUIterationDecision, CPU},
        keyboard::parse_key_code,
    },
    sound::{beep::Sound, message::SoundMessage},
};

use log::{debug, error, info};

const SCALING_FACTOR: u32 = 10;
const DISPLAY_ROWS: u32 = 32;
const DISPLAY_COLUMNS: u32 = 64;

/// A Work-In-Progress CHIP-8 emulator
#[derive(Parser, Debug)]
#[command(name = "Chippy")]
#[command(author = "Salvio G. <sungvzer@proton.me>")]
#[command(version = "0.1.0")]
#[command(about, long_about = None)]
struct Cli {
    /// .ch8 file to load program from
    #[arg(short, long, required = true)]
    file: PathBuf,

    /// Keymap .json file
    #[arg(short, long)]
    keymap: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,
}

fn create_window(width: f64, height: f64, event_loop: &EventLoop<()>) -> Window {
    let mut file_menu = MenuBar::new();
    file_menu.add_native_item(MenuItem::Quit);

    let mut menu = MenuBar::new();
    menu.add_submenu("File", true, file_menu);

    let size = LogicalSize::new(width, height);

    let builder = WindowBuilder::new();

    builder
        .with_title("Chippy")
        .with_menu(menu)
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(event_loop)
        .unwrap()
}

fn exit(timer_tick_stop: Arc<AtomicBool>, control_flow: &mut ControlFlow) {
    timer_tick_stop.store(true, Ordering::Relaxed);
    *control_flow = ControlFlow::Exit;
}

fn handle_window_event(
    event: WindowEvent,
    pixels: &mut Pixels,
    control_flow: &mut ControlFlow,
    cpu: &mut CPU,
    timer_tick_stop: Arc<AtomicBool>,
    keymap: &Keymap,
) {
    match event {
        WindowEvent::Resized(size) => {
            pixels.resize_surface(size.width, size.height).unwrap();
            pixels.render().unwrap();
        }
        WindowEvent::CloseRequested => {
            exit(timer_tick_stop, control_flow);
        }
        WindowEvent::KeyboardInput { event, .. } => {
            if event.physical_key == KeyCode::Escape {
                exit(timer_tick_stop, control_flow);
            }

            let relevant = parse_key_code(event.physical_key, &keymap.keys);

            if event.state == ElementState::Released || relevant.is_none() {
                cpu.set_key_pressed(None);
            } else {
                cpu.set_key_pressed(relevant);
            };
        }

        _ => {}
    };
}

fn init_60hz_clock(tx: Sender<u64>, stop_signal: Arc<AtomicBool>) -> thread::JoinHandle<()> {
    let mut ticks = 0;
    let clock_closure = move || loop {
        if stop_signal.load(Ordering::Relaxed) {
            break;
        }
        thread::sleep(Duration::from_millis(16));
        tx.send(ticks).expect("Could not send the tick");
        ticks += 1;
    };
    thread::spawn(clock_closure)
}

fn init_beep(message_rx: Receiver<SoundMessage>) -> thread::JoinHandle<()> {
    let closure = move || {
        let _beep = Sound::new(message_rx);
    };
    thread::spawn(closure)
}

fn main() -> Result<(), String> {
    let args = Cli::parse();
    debug!("Parsed CLI arguments");

    let (clock_tx, clock_rx) = mpsc::channel();
    let (sound_message_tx, sound_message_rx) = mpsc::channel();
    sound_message_tx
        .send(SoundMessage::Stop)
        .unwrap_or_else(|err| {
            error!("Error stopping sound: {:?}", err);
        });
    let timer_tick_stop = Arc::new(AtomicBool::new(false));

    let join_clock = init_60hz_clock(clock_tx, Arc::clone(&timer_tick_stop));
    let join_sound = init_beep(sound_message_rx);

    let mut cpu = CPU::new(sound_message_tx);

    match logs::log_init(args.debug) {
        Ok(()) => {
            info!("Logger setup successfully")
        }
        Err(error) => {
            println!("Could not setup logger: {}", error)
        }
    };

    let keymap: Keymap = if let Some(keymap) = args.keymap {
        keymap::read_keymap(keymap).unwrap()
    } else {
        keymap::default_keymap()
    };
    println!("{:?}", keymap);

    cpu.load_program_from_file(args.file)?;

    // GUI Init
    let event_loop = EventLoop::new();
    let window_width = DISPLAY_COLUMNS * SCALING_FACTOR;
    let window_height = DISPLAY_ROWS * SCALING_FACTOR;

    let window = create_window(window_width as f64, window_height as f64, &event_loop);
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(64, 32, surface_texture).unwrap()
    };

    // We do this to avoid the compiler screaming at us for moving the handle
    let mut join_clock_option = Some(join_clock);
    let mut join_sound_option = Some(join_sound);

    event_loop.run(move |event, _target, control_flow| {
        if let Ok(tick) = clock_rx.try_recv() {
            cpu.tick(tick);
        }
        match event {
            Event::WindowEvent { event, .. } => {
                handle_window_event(
                    event,
                    &mut pixels,
                    control_flow,
                    &mut cpu,
                    timer_tick_stop.clone(),
                    &keymap,
                );
                if *control_flow == ControlFlow::Exit {
                    debug!("Joining 60Hz clock thread");
                    join_clock_option.take().map(JoinHandle::join);

                    debug!("Joining sound thread");
                    cpu.force_audio_stop();
                    join_sound_option.take().map(JoinHandle::join);
                }
            }
            Event::MainEventsCleared => {
                if let CPUIterationDecision::Halt = cpu.fetch_decode_execute() {
                    *control_flow = ControlFlow::Wait;
                };
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let frame = pixels.get_frame_mut();
                if cpu.screen_mut().draw(frame) {
                    pixels.render().unwrap();
                }
            }
            _ => {}
        };
    });
}
