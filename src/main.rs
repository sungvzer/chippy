pub mod chip8;

use chrono::{DateTime, Utc};
use clap::{arg, command, Parser};
use fern::colors::{Color, ColoredLevelConfig};

use std::{path::PathBuf, time::SystemTime};

use chip8::{
    cpu::{CPUIterationDecision, CPU},
    dumper::{dump_cpu, DumpMemory},
    gfx::screen::Screen,
};

use log::{error, info};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

/// A Work-In-Progress CHIP-8 emulator
#[derive(Parser, Debug)]
#[command(name = "Chippy")]
#[command(author = "Salvio G. <sungvzer@proton.me>")]
#[command(version = "0.1.0")]
#[command(about, long_about = None)]
struct Cli {
    /// .ch8 file to load program from
    file: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,

    /// Enable the screen (should always be on unless you really want to disable it)
    #[arg(short, long)]
    gui: bool,
}

fn log_init(debug_enabled: bool) -> Result<(), log::SetLoggerError> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .debug(Color::Blue)
        .error(Color::Red);

    let today: DateTime<Utc> = SystemTime::now().into();
    let today = today.format("%Y-%m-%d");
    let filename = format!("log-{}.log", today);

    let stdout_dispatcher = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}:{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                colors.color(record.level()),
                message
            ))
        })
        .chain(std::io::stdout());

    let file_dispatcher = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}:{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(filename).unwrap());

    fern::Dispatch::new()
        // Ignore non-interesting logs from other sources
        .level(log::LevelFilter::Warn)
        // Keep our logs
        .level_for(
            "chip8",
            if debug_enabled {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Info
            },
        )
        .chain(stdout_dispatcher)
        .chain(file_dispatcher)
        .apply()?;
    Ok(())
}

fn run_gui() -> Result<(), pixels::Error> {
    let event_loop = EventLoop::new();
    let screen = Screen::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(Screen::WIDTH as f64, Screen::HEIGHT as f64);
        let scaled_size =
            LogicalSize::new(Screen::WIDTH as f64 * 10.0, Screen::HEIGHT as f64 * 10.0);
        WindowBuilder::new()
            .with_title("CHIP-8")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(Screen::WIDTH as u32, Screen::HEIGHT as u32, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            let frame = pixels.get_frame_mut();

            screen.draw(frame);

            if let Err(err) = pixels.render() {
                error!("pixels.render() failed: {}", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our state and request a redraw.
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            window.request_redraw();
        }
    });
}

fn main() -> Result<(), String> {
    let args = Cli::parse();

    match log_init(args.debug) {
        Ok(()) => {
            info!("Logger setup successfully")
        }
        Err(error) => {
            println!("Could not setup logger: {}", error.to_string())
        }
    };
    let mut cpu = CPU::new();

    if let Some(file_name) = args.file {
        cpu.load_program_from_file(file_name)?;
        dump_cpu(&cpu, DumpMemory::Yes);
    }

    while let CPUIterationDecision::Continue = cpu.fetch_decode_execute() {}

    if args.gui {
        if let Err(err) = run_gui() {
            let string = format!("{:?}", err);
            error!("{}", string);
            return Err("Generic GUI error. Check logs".to_string());
        }
    }

    Ok(())
}
