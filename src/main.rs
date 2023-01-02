#![forbid(unsafe_code)]
#![deny(clippy::all)]
pub mod chip8;
mod logs;

use clap::{arg, command, Parser};

use std::path::PathBuf;

use chip8::cpu::cpu::{CPUIterationDecision, CPU};

use log::{debug, info};

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

    /// Enable the screen (should always be on unless you really want to disable it)
    #[arg(short, long)]
    gui: bool,
}

fn main() -> Result<(), String> {
    let args = Cli::parse();
    debug!("Parsed CLI arguments");

    match logs::log_init(args.debug) {
        Ok(()) => {
            info!("Logger setup successfully")
        }
        Err(error) => {
            println!("Could not setup logger: {}", error)
        }
    };
    let mut cpu = CPU::new();

    cpu.load_program_from_file(args.file)?;

    while let CPUIterationDecision::Continue = cpu.fetch_decode_execute() {}

    Ok(())
}
