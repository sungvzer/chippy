use std::fmt::Write;
use std::io::ErrorKind;
use std::panic;
use std::{
    fs::{create_dir, OpenOptions},
    path::Path,
    time::SystemTime,
};

use log::{debug, error};

use super::cpu::CPU;

#[derive(PartialEq, Eq)]
pub enum DumpMemory {
    No,
    Yes,
}
fn get_time_str() -> String {
    let now: chrono::DateTime<chrono::Utc> = SystemTime::now().into();
    now.to_rfc3339()
}

pub fn dump_cpu(cpu: &CPU, should_dump_memory: DumpMemory) {
    if should_dump_memory == DumpMemory::Yes {
        dump_memory(cpu);
    }
    dump_registers(cpu);
}

fn dump_registers(cpu: &CPU) {
    let registers = cpu.registers();
    debug!("Registers");
    let mut str = "".to_string();
    for (i, register) in registers.iter().enumerate() {
        let mut formatted = format!("V{:X} = 0x{:02x}", i, register);
        formatted += ", ";
        str.push_str(&formatted);
    }

    write!(&mut str, "SP = 0x{:02x}, ", cpu.stack_pointer()).unwrap();
    write!(&mut str, "PC = 0x{:02x}, ", cpu.program_counter()).unwrap();

    write!(&mut str, "I = 0x{:02x}", cpu.memory_location()).unwrap();

    debug!("{}", str);
}

fn dump_memory(cpu: &CPU) {
    let dir = std::env::temp_dir();

    let dir = dir.join(Path::new("chip8-dump"));

    let dir_string = dir.to_str().unwrap().to_string();
    if let Err(error) = create_dir(dir_string.as_str()) {
        if error.kind() != ErrorKind::AlreadyExists {
            panic!("{}", error);
        }
    };

    let file_name = format!("{}.bin", get_time_str());

    let dir = dir.join(file_name);

    let dir_string = dir.to_str().unwrap().to_string();
    debug!("Memory dump file path: {dir_string}");

    let mut file = match OpenOptions::new().write(true).create(true).open(dir) {
        Ok(file) => file,
        Err(err) => {
            error!("Error: {}", err.to_string());
            return;
        }
    };

    let mem = cpu.memory();
    match std::io::Write::write(&mut file, mem) {
        Ok(_) => {
            debug!("Dumped memory successfully");
        }
        Err(err) => {
            error!("Could not dump memory: {:?}", err);
        }
    };
}
