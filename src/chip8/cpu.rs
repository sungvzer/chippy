use std::{fs::OpenOptions, io::Read, path::PathBuf};

use log::{debug, error, info, warn};

type Register = u8;

pub const V0: Register = 0x0;
pub const V1: Register = 0x1;
pub const V2: Register = 0x2;
pub const V3: Register = 0x3;
pub const V4: Register = 0x4;
pub const V5: Register = 0x5;
pub const V6: Register = 0x6;
pub const V7: Register = 0x7;
pub const V8: Register = 0x8;
pub const V9: Register = 0x9;
pub const VA: Register = 0xA;
pub const VB: Register = 0xB;
pub const VC: Register = 0xC;
pub const VD: Register = 0xD;
pub const VE: Register = 0xE;
pub const VF: Register = 0xF;

pub enum CPUIterationDecision {
    Continue,
    Halt,
}

pub struct CPU {
    memory: [u8; 4096], // TODO: Maybe move to a different struct and datatype

    registers: [u8; 16],

    /// NOTE: Only 12 bits are used for this
    memory_location: u16,

    /// NOTE: Only 12 bits are used for this
    program_counter: u16,

    /// NOTE: Only 12 bits are used for this
    stack_pointer: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0x0; 16],
            memory_location: 0x0,
            program_counter: 0x0,
            stack_pointer: 0x0,
            memory: [0xff; 4096],
        }
    }

    pub fn load_program_from_file(&mut self, file_path: PathBuf) -> Result<usize, String> {
        debug!("Loading file {}", file_path.as_path().to_str().unwrap());

        if !file_path
            .extension()
            .unwrap_or_default()
            .eq_ignore_ascii_case("ch8")
        {
            error!("File extension is not ch8");
            return Err("Wrong file extension".to_string());
        }
        let file = match OpenOptions::new().read(true).open(file_path) {
            Ok(file) => Some(file),
            Err(err) => {
                error!("Could not open file: {}", err.kind().to_string());
                return Err("Could not open file".to_string());
            }
        };

        if file.is_none() {
            warn!("Did not provide a file. Exiting...");
            return Err("Did not provide a file".to_string());
        }

        let file = file.unwrap();

        let mut buffer = vec![];
        let bytes = file.take(1024).read_to_end(&mut buffer).unwrap();

        let mut index = 512;
        for byte in buffer {
            self.memory[index] = byte;
            index += 1;
        }

        info!("Loaded {} bytes program into memory", bytes);

        self.program_counter = 0x200;

        Ok(bytes)
    }

    pub fn stack_pointer(&self) -> u16 {
        self.stack_pointer
    }
    pub fn program_counter(&self) -> u16 {
        self.program_counter
    }
    pub fn memory_location(&self) -> u16 {
        self.memory_location
    }

    pub fn registers(&self) -> [u8; 16] {
        let reg = self.registers.to_owned();
        reg
    }

    pub fn set_register(&mut self, register: Register, value: u8) -> () {
        self.registers[register as usize] = value;
    }

    pub fn get_register(&self, register: Register) -> u8 {
        self.registers[register as usize]
    }

    pub fn memory(&self) -> &[u8; 4096] {
        &self.memory
    }

    pub fn fetch_decode_execute(&mut self) -> CPUIterationDecision {
        let first_half: u16 = self.memory[self.program_counter as usize].into();
        let second_half: u16 = self.memory[self.program_counter as usize + 1].into();
        let mut instruction = first_half << 8;
        instruction |= second_half;

        debug!("Instruction: {:04x}", instruction);

        if let 0xFFFF = instruction {
            debug!("Halting");
            return CPUIterationDecision::Halt;
        }

        self.program_counter += 2;
        return CPUIterationDecision::Continue;
    }
}
