#![forbid(unsafe_code)]
#![deny(clippy::all)]
pub mod chip8;
mod logs;

use clap::{arg, command, Parser};
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
        mpsc::{self, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use chip8::cpu::{
    cpu::{CPUIterationDecision, CPU},
    keyboard::is_relevant_key_code,
};

use log::{debug, info};

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
) {
    match event {
        WindowEvent::Resized(size) => {
            pixels.resize_surface(size.width, size.height).unwrap();
        }
        WindowEvent::CloseRequested => {
            exit(timer_tick_stop, control_flow);
        }
        WindowEvent::KeyboardInput { event, .. } => {
            if event.physical_key == KeyCode::Escape {
                exit(timer_tick_stop, control_flow);
            }

            let relevant = is_relevant_key_code(event.physical_key);

            if event.state == ElementState::Released || !relevant {
                cpu.set_key_pressed(None);
            } else {
                cpu.set_key_pressed(Some(event.physical_key));
            };
        }

        _ => {}
    };
}

fn init_60hz_clock(tx: Sender<u64>, stop_signal: Arc<AtomicBool>) -> thread::JoinHandle<()> {
    let mut ticks = 0;
    let tick_closure = move || loop {
        if stop_signal.load(Ordering::Relaxed) == true {
            break;
        }
        thread::sleep(Duration::from_millis(16));
        tx.send(ticks).expect("Could not send the tick");
        ticks += 1;
    };
    let ticker = thread::spawn(tick_closure);
    ticker
}

fn main() -> Result<(), String> {
    let args = Cli::parse();
    debug!("Parsed CLI arguments");

    let (timer_tick_tx, timer_tick_rx) = mpsc::channel();
    let timer_tick_stop = Arc::new(AtomicBool::new(false));

    let join_handle_ticker = init_60hz_clock(timer_tick_tx, Arc::clone(&timer_tick_stop));

    let mut cpu = CPU::new();
    cpu.load_program_from_file(args.file)?;

    match logs::log_init(args.debug) {
        Ok(()) => {
            info!("Logger setup successfully")
        }
        Err(error) => {
            println!("Could not setup logger: {}", error)
        }
    };

    // GUI Init
    let event_loop = EventLoop::new();
    let window_width = DISPLAY_COLUMNS * SCALING_FACTOR;
    let window_height = DISPLAY_ROWS * SCALING_FACTOR;

    let window = create_window(window_width as f64, window_height as f64, &event_loop);
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(64, 32, surface_texture).unwrap();
        pixels
    };

    // We do this to avoid the compiler screaming at us for moving the handle
    let mut join_option = Some(join_handle_ticker);

    event_loop.run(move |event, _target, control_flow| {
        if let Ok(tick) = timer_tick_rx.try_recv() {
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
                );
                if *control_flow == ControlFlow::Exit {
                    debug!("Joining 60Hz clock thread");
                    join_option.take().map(JoinHandle::join);
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
                cpu.screen().draw(frame);
                pixels.render().unwrap();
            }
            _ => {}
        };
    });
}
