#![forbid(unsafe_code)]
#![deny(clippy::all)]
pub mod chip8;
mod logs;

use clap::{arg, command, Parser};
use pixels::{Pixels, SurfaceTexture};
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    menu::{MenuBar, MenuItem},
    window::{Window, WindowBuilder},
};

use std::path::PathBuf;

use chip8::cpu::cpu::{CPUIterationDecision, CPU};

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

fn handle_window_event(event: WindowEvent, pixels: &mut Pixels, control_flow: &mut ControlFlow) {
    match event {
        WindowEvent::Resized(size) => {
            pixels.resize_surface(size.width, size.height).unwrap();
        }
        WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::Exit;
        }
        WindowEvent::KeyboardInput { event, .. } => {
            if event.physical_key == KeyCode::Escape {
                *control_flow = ControlFlow::Exit;
            }
        }

        _ => {}
    };
}

fn main() -> Result<(), String> {
    let args = Cli::parse();
    debug!("Parsed CLI arguments");

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

    event_loop.run(move |event, _target, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => {
                handle_window_event(event, &mut pixels, control_flow);
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
