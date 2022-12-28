use std::{
    fs::{create_dir, OpenOptions},
    io::Write,
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

fn dump_registers(cpu: &CPU) -> () {
    let registers = cpu.registers();
    debug!("Registers");
    let mut str = "".to_string();
    for i in 0..registers.len() {
        let mut formatted = format!("V{:X} = 0x{:02x}", i, registers[i]);
        formatted += ", ";
        str.push_str(&formatted);
    }

    str.push_str(&format!("SP = 0x{:02x}, ", cpu.stack_pointer()));
    str.push_str(&format!("PC = 0x{:02x}, ", cpu.program_counter()));
    str.push_str(&format!("I = 0x{:02x}", cpu.memory_location()));

    debug!("{}", str);
}

fn dump_memory(cpu: &CPU) -> () {
    let dir = std::env::temp_dir();

    let dir = dir.join(Path::new("chip8-dump"));

    let dir_string = dir.to_str().unwrap().to_string();
    match create_dir(dir_string.as_str()) {
        Ok(_) => {}
        Err(_) => {}
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
    match file.write(mem) {
        Ok(_) => {
            debug!("Dumped memory successfully");
        }
        Err(err) => {
            error!("Could not dump memory: {:?}", err);
        }
    };
}
