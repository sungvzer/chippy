pub mod chip8;

use chrono::{DateTime, Utc};
use clap::{arg, command, Parser};
use fern::colors::{Color, ColoredLevelConfig};

use std::{path::PathBuf, time::SystemTime};

use chip8::cpu::{CPUIterationDecision, CPU};

use log::{debug, info};

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

fn main() -> Result<(), String> {
    let args = Cli::parse();
    debug!("Parsed CLI arguments");

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
    }

    while let CPUIterationDecision::Continue = cpu.fetch_decode_execute() {}

    Ok(())
}
