pub mod instruction;
pub mod rng;

use std::{fs::OpenOptions, io::Read, path::PathBuf};

use log::{debug, error, info, warn};

use crate::chip8::{
    cpu::{
        instruction::{parse_instruction, Instruction},
        rng::random_byte,
    },
    dumper::{dump_cpu, DumpMemory},
};

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

    stack_pointer: u8,
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

    pub fn stack_pointer(&self) -> u8 {
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

    fn read_u16_from_memory(&self, addr: usize) -> u16 {
        ((self.memory[addr] as u16) << 8) | self.memory[addr + 1] as u16
    }

    fn jump(&mut self, addr: u16) {
        self.program_counter = addr;
    }

    pub fn fetch_decode_execute(&mut self) -> CPUIterationDecision {
        // Fetch
        let instruction_opcode = self.read_u16_from_memory(self.program_counter as usize);

        // Decode
        let instruction = match parse_instruction(instruction_opcode) {
            Ok(instruction) => instruction,
            Err(_) => {
                dump_cpu(&self, DumpMemory::No);
                let string = format!("Un-parsed opcode: {:04X}", instruction_opcode);
                error!("{}", string);
                todo!("{}", string);
            }
        };

        // Execute
        match instruction {
            Instruction::JP(addr) => {
                debug!("JP {:04X}", addr);
                self.jump(addr)
            }
            Instruction::LD(register, value) => {
                debug!("LD V{:X}, {:02X}", register, value);
                self.set_register(register, value);
            }
            Instruction::HLT => {
                debug!("Halting");
                return CPUIterationDecision::Halt;
            }
            Instruction::RND(register, and_mask) => {
                let byte = random_byte();
                debug!("RND V{:X}, 0x{:02X} & 0x{:02X}", register, byte, and_mask);
                self.set_register(register, byte & and_mask);
            }
            Instruction::CLS => {
                debug!("TODO: Implement CLS");
            }
            other => {
                todo!("Implement {:?}", other)
            }
        }
        self.program_counter += 2;

        return CPUIterationDecision::Continue;
    }
}
