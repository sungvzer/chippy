use std::fmt::Write as _;
use std::io::Write as _;
use std::{
    ffi::OsStr,
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use chip8::cpu::instruction::{parse_instruction, Instruction, InstructionParseResult};
use clap::{command, Parser, ValueEnum};
use log::error;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum DisassembleFormat {
    /// Get only the hex dump of the file
    Hex,

    /// Get the instruction disassembly (e.g. DRW (VA, VB, 3))
    Instructions,

    /// Get the full instruction disassembly (Instruction + Hex)
    Full,
}

/// Disassemble a .ch8 file to hex or to instructions
#[derive(Parser)]
#[command(name = "Chip-8 Disassembler")]
#[command(author = "Salvio G. <sungvzer@proton.me>")]
#[command(version = "0.1.0")]
#[command(about, long_about = None)]

struct Cli {
    /// Input .ch8 file path
    #[arg(short, long = "input", required = true)]
    input_file: PathBuf,

    /// Output disassembled file path
    #[arg(short, long = "output", required = true)]
    output_file: PathBuf,

    /// Show expected instruction memory address
    #[arg(value_enum, short, long = "address")]
    show_memory_address: bool,

    /// Disassembly format
    #[arg(value_enum, short, long, default_value_t = DisassembleFormat::Hex)]
    format: DisassembleFormat,
}

fn write_file_hex(path: PathBuf, bytes: &Vec<u8>, show_memory_address: bool) -> Result<(), String> {
    let mut file = match OpenOptions::new().create(true).write(true).open(path) {
        Ok(file) => file,
        Err(err) => {
            error!("Could not open file: {}", err.kind().to_string());
            return Err("Could not open file".to_string());
        }
    };

    let mut address = 0x200;

    for byte in bytes.chunks(2) {
        let mut formatted_string = "".to_string();
        if show_memory_address {
            write!(formatted_string, "{:3X} - ", address).unwrap();
        }

        if byte.len() == 2 {
            write!(formatted_string, "{:02X}{:02X}", byte[0], byte[1]).unwrap();
        } else {
            write!(formatted_string, "{:02X}", byte[0]).unwrap();
        }

        match write!(&mut file, "{}\n", formatted_string) {
            Ok(_) => {}
            Err(err) => return Err(format!("Error writing to file: {}", err.kind().to_string())),
        };
        address += 2;
    }

    Ok(())
}
fn write_file_instructions(
    path: PathBuf,
    instructions: &Vec<Instruction>,
    show_memory_address: bool,
) -> Result<(), String> {
    let mut file = match OpenOptions::new().create(true).write(true).open(path) {
        Ok(file) => file,
        Err(err) => {
            error!("Could not open file: {}", err.kind().to_string());
            return Err("Could not open file".to_string());
        }
    };

    let mut address = 0x200;

    for instruction in instructions {
        let mut formatted_string = "".to_string();
        if show_memory_address {
            write!(formatted_string, "{:3X} - ", address).unwrap();
        }

        write!(formatted_string, "{:?}", instruction).unwrap();

        match write!(&mut file, "{}\n", formatted_string) {
            Ok(_) => {}
            Err(err) => return Err(format!("Error writing to file: {}", err.kind().to_string())),
        };
        address += 2;
    }

    Ok(())
}

fn write_file_full(
    path: PathBuf,
    bytes: &Vec<u8>,
    show_memory_address: bool,
) -> Result<(), String> {
    let mut file = match OpenOptions::new().create(true).write(true).open(path) {
        Ok(file) => file,
        Err(err) => {
            error!("Could not open file: {}", err.kind().to_string());
            return Err("Could not open file".to_string());
        }
    };
    let instructions = disassemble(bytes);

    let mut address = 0x200;
    for (instruction, op_code) in instructions.iter().zip(bytes.chunks_exact(2)) {
        let mut formatted_string = "".to_string();
        if show_memory_address {
            write!(formatted_string, "{:03X} - ", address).unwrap();
        }

        if op_code.len() == 2 {
            write!(formatted_string, "{:02X}{:02X}", op_code[0], op_code[1]).unwrap();
        } else {
            write!(formatted_string, "{:02X}", op_code[0]).unwrap();
        };

        match write!(&mut file, "{} - {:?}\n", formatted_string, instruction) {
            Ok(_) => {}
            Err(err) => return Err(format!("Error writing to file: {}", err.kind().to_string())),
        };
        address += 2;
    }

    Ok(())
}

fn read_file<F>(path: PathBuf, validate_ext: F) -> Result<Vec<u8>, String>
where
    F: Fn(&OsStr) -> bool,
{
    let ext = path.extension().unwrap_or_default();
    if !validate_ext(ext) {
        return Err("Wrong file extension".to_string());
    }

    let mut file = match OpenOptions::new().read(true).open(path) {
        Ok(file) => file,
        Err(err) => {
            error!("Could not open file: {}", err.kind().to_string());
            return Err("Could not open file".to_string());
        }
    };

    let mut buffer = vec![];
    match file.seek(SeekFrom::Start(0)) {
        Ok(_) => {}
        Err(err) => {
            error!("Could not seek into file: {}", err.kind().to_string());
            return Err("Could not seek into file".to_string());
        }
    };
    match file.read_to_end(&mut buffer) {
        Ok(_) => {}
        Err(err) => {
            error!("Could not read file: {}", err.kind().to_string());
            return Err("Could not read file".to_string());
        }
    };
    Ok(buffer)
}

fn disassemble(input_bytes: &Vec<u8>) -> Vec<Instruction> {
    let mut result = Vec::new();
    for vector in input_bytes.chunks_exact(2) {
        let number = ((vector[0] as u16) << 8) | vector[1] as u16;
        result.push(match parse_instruction(number) {
            InstructionParseResult::Ok(instruction) => instruction,
            InstructionParseResult::Unparsed => {
                panic!("Cannot parse instruction: {:04X}", number);
            }
        })
    }
    result
}

fn main() -> Result<(), String> {
    let args = Cli::parse();
    let show_memory_address = args.show_memory_address;

    let input_file = args.input_file;
    let output_file = args.output_file;

    let input_bytes = read_file(input_file, |str| str.eq_ignore_ascii_case("ch8")).unwrap();

    if args.format == DisassembleFormat::Hex {
        return write_file_hex(output_file, &input_bytes, show_memory_address);
    } else {
        let instructions = disassemble(&input_bytes);
        if args.format == DisassembleFormat::Instructions {
            return write_file_instructions(output_file, &instructions, show_memory_address);
        } else {
            return write_file_full(output_file, &input_bytes, show_memory_address);
        }
    }
}
