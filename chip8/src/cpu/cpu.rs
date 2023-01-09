use crate::{
    cpu::{keyboard::key_code_to_u8, sprites::get_sprite},
    gfx::screen::Screen,
    sound::message::SoundMessage,
};
use std::{fs::OpenOptions, io::Read, path::PathBuf, sync::mpsc::Sender};

use log::{debug, error, info, warn};
use tao::keyboard::KeyCode;

use crate::{
    cpu::{
        instruction::{parse_instruction, Instruction},
        rng::random_byte,
    },
    dumper::{dump_cpu, DumpMemory},
};

use super::{
    instruction::InstructionParseResult,
    timer::{delay_timer::DelayTimer, sound_timer::SoundTimer, timer::Timer},
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
    stack: [u16; 16],

    /// NOTE: Only 12 bits are used for this
    memory_location: u16,

    /// NOTE: Only 12 bits are used for this
    program_counter: u16,

    stack_pointer: u8,

    /// Stores the currently pressed key
    active_key_code: Option<KeyCode>,

    waiting_for_key_press: bool,

    screen: Screen,

    delay_timer: DelayTimer,
    sound_timer: SoundTimer,
}

impl CPU {
    pub fn tick(&mut self, _: u64) {
        self.sound_timer.tick();
        self.delay_timer.tick();
    }

    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    pub fn screen_mut(&mut self) -> &mut Screen {
        &mut self.screen
    }

    pub fn new(sound_tx: Sender<SoundMessage>) -> Self {
        let mut cpu = CPU {
            registers: [0x0; 16],
            memory_location: 0x0,
            program_counter: 0x0,
            stack_pointer: 0x0,
            memory: [0xff; 4096],
            screen: Screen::new(),
            active_key_code: None,
            waiting_for_key_press: false,
            stack: [0x0; 16],
            delay_timer: DelayTimer::new(),
            sound_timer: SoundTimer::new(sound_tx),
        };
        cpu.initialize_sprites();
        cpu.clear_screen();

        cpu
    }

    fn initialize_sprites(&mut self) {
        for i in 0..=15 {
            let sprite = get_sprite(i);
            let index = i * 5;
            for j in index..index + 5 {
                let sprite_index = j - index;
                self.memory[j as usize] = sprite[sprite_index as usize];
            }
        }
        debug!("Initialized sprites from address 0x000 to address 0x04F");
    }

    fn get_sprite_address(&self, sprite: u8) -> u16 {
        sprite as u16 * 5
    }

    fn clear_screen(&mut self) {
        self.screen.clear();
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
        self.registers.to_owned()
    }

    pub fn set_register(&mut self, register: Register, value: u8) {
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

    fn copy_register_bcd_into_memory(&mut self, register: u8) {
        let mut value = self.registers[register as usize];

        let addr = self.memory_location;

        let ones = value % 10;
        value /= 10;
        let tens = value % 10;
        value /= 10;
        let hundreds = value % 10;
        self.memory[addr as usize] = hundreds;
        self.memory[(addr + 1) as usize] = tens;
        self.memory[(addr + 2) as usize] = ones;
    }

    fn jump(&mut self, addr: u16) {
        self.program_counter = addr;
    }

    fn call(&mut self, addr: u16) {
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;

        self.program_counter = addr;
    }

    fn ret(&mut self) {
        let index = (self.stack_pointer - 1) as usize;
        self.program_counter = self.stack[index];
        self.stack_pointer -= 1;
    }

    pub fn set_key_pressed(&mut self, key: Option<KeyCode>) {
        self.active_key_code = key;
    }

    pub fn fetch_decode_execute(&mut self) -> CPUIterationDecision {
        if self.waiting_for_key_press && self.active_key_code == Option::None {
            return CPUIterationDecision::Continue;
        }

        // Fetch
        let instruction_opcode = self.read_u16_from_memory(self.program_counter as usize);

        // Decode
        let instruction = match parse_instruction(instruction_opcode) {
            InstructionParseResult::Ok(instruction) => instruction,
            InstructionParseResult::Unparsed => {
                dump_cpu(self, DumpMemory::No);
                let string = format!("Un-parsed opcode: {:04X}", instruction_opcode);
                error!("{}", string);
                todo!("{}", string);
            }
        };

        // Execute
        match instruction {
            Instruction::RET => {
                debug!("RET");
                self.ret();
            }
            Instruction::JP(addr) => {
                debug!("JP {:04X}", addr);
                self.jump(addr);
                return CPUIterationDecision::Continue;
            }
            Instruction::CALL(addr) => {
                debug!("CALL {:04X}", addr);
                self.call(addr);
                return CPUIterationDecision::Continue;
            }
            Instruction::LDI(addr) => {
                debug!("LD I, {:04X}", addr);
                self.memory_location = addr;
            }
            Instruction::LD(register, value) => {
                debug!("LD V{:X}, {:02X}", register, value);
                self.set_register(register, value);
            }
            Instruction::ADD(register, value) => {
                debug!("ADD V{:X}, {:02X}", register, value);
                let register_value = self.get_register(register);
                let result = register_value.wrapping_add(value);

                self.set_register(register, result);
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
                debug!("CLS");
                self.clear_screen();
            }
            Instruction::LDB(register) => {
                debug!("LD B, V{:X}", register);
                self.copy_register_bcd_into_memory(register);
            }
            Instruction::LDIFromVx(register) => {
                debug!("LD [I], V{:X}", register);
                let memory = self.memory_location;
                for i in 0..=register {
                    let value = self.get_register(i);
                    let memory_index = (memory + i as u16) as usize;
                    self.memory[memory_index] = value;
                }
            }
            Instruction::LDVxFromK(register) => {
                debug!("LD V{:X}, K", register);
                if let Some(key) = self.active_key_code {
                    debug!("Key pressed! {:?}", key);
                    self.set_register(register, key_code_to_u8(key));
                    self.waiting_for_key_press = false;
                } else {
                    self.waiting_for_key_press = true;
                    return CPUIterationDecision::Continue;
                }
            }
            Instruction::LDVxFromI(register) => {
                debug!("LD V{:X}, [I]", register);
                let memory = self.memory_location;
                for i in 0..=register {
                    let memory_index = (memory + i as u16) as usize;
                    self.set_register(i, self.memory[memory_index]);
                }
            }
            Instruction::LDF(register) => {
                debug!("LD F, V{:X}", register);
                let sprite = self.get_register(register);
                let sprite_start = self.get_sprite_address(sprite);
                self.memory_location = sprite_start;
            }
            Instruction::DRW(x, y, byte_length) => {
                debug!("DRW V{:X}, V{:X}, {:X}", x, y, byte_length);

                // usize casting
                let x = self.get_register(x) as usize;
                let y = self.get_register(y) as usize;
                let byte_length = byte_length as usize;
                let sprite_address = self.memory_location as usize;
                let range = sprite_address..(sprite_address + byte_length);

                let sprite = &self.memory[range];
                let did_erase = self.screen.draw_sprite(x, y, &sprite.to_vec());
                self.set_register(VF, if did_erase { 1 } else { 0 });
                debug!("Drawn sprite to screen");
            }
            Instruction::SE(register, value) => {
                debug!("SE V{:X}, {:02X}", register, value);
                if self.get_register(register) == value {
                    self.program_counter += 2;
                }
            }
            Instruction::SNE(register, value) => {
                debug!("SNE V{:X}, {:02X}", register, value);
                if self.get_register(register) != value {
                    self.program_counter += 2;
                }
            }
            Instruction::LDVxFromVy(x, y) => {
                debug!("LD V{:X}, V{:X}", x, y);
                self.set_register(x, self.get_register(y));
            }
            Instruction::LDDTFromVx(register) => {
                debug!("LD DT, V{:X}", register);
                let value = self.get_register(register);
                self.delay_timer.set_value(value);
            }
            Instruction::LDSTFromVx(register) => {
                debug!("LD ST, V{:X}", register);
                let value = self.get_register(register);
                self.sound_timer.set_value(value);
            }
            Instruction::LDVxFromDT(register) => {
                debug!("LD V{:X}, DT", register);
                let value = self.delay_timer.get_value();
                self.set_register(register, value);
            }
            Instruction::AND(x, y) => {
                debug!("AND V{:X}, V{:X}", x, y);
                let vx = self.get_register(x);
                let vy = self.get_register(y);
                self.set_register(x, vx & vy);
            }

            Instruction::ADDIVx(register) => {
                debug!("ADD I, V{:X}", register);
                let vx = self.get_register(register);
                self.memory_location += vx as u16;
            }

            Instruction::SKP(register) => {
                debug!("SKP V{:X}", register);
                let value = self.get_register(register);
                let active_key_code = self.active_key_code;
                if let Some(key) = active_key_code {
                    let key = key_code_to_u8(key);
                    if key == value {
                        self.program_counter += 2;
                    }
                }
            }
            Instruction::SKNP(register) => {
                debug!("SKNP V{:X}", register);
                let value = self.get_register(register);
                let active_key_code = self.active_key_code;
                if active_key_code == None || key_code_to_u8(active_key_code.unwrap()) != value {
                    self.program_counter += 2;
                }
            }
            Instruction::SUB(x, y) => {
                debug!("SUB V{:X}, V{:X}", x, y);
                let vx = self.get_register(x);
                let vy = self.get_register(y);

                let not_borrow: u8 = if vx > vy { 1 } else { 0 };
                self.set_register(VF, not_borrow);

                let sub_result = vx.wrapping_sub(vy);
                self.set_register(x, sub_result);
            }

            other => {
                debug!("TODO: Implement {:?}", other);
            }
        }
        self.program_counter += 2;

        CPUIterationDecision::Continue
    }

    pub fn force_audio_stop(&self) {
        self.sound_timer.force_audio_stop();
    }
}
