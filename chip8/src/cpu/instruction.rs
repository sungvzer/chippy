use core::fmt::Debug;

pub enum Instruction {
    /** JP addr */
    JP(u16),

    /** CLS */
    CLS,

    /** RET */
    RET,

    /** CALL nnn - Call subroutine at nnn */
    CALL(u16),

    /** SE Vx, byte - Skip next instruction if Vx == byte */
    SE(u8, u8),

    /** SE Vx, Vy - Skip next instruction if Vx == Vy */
    SEVxVy(u8, u8),
    /** SNE Vx, Vy - Skip next instruction if Vx != Vy */
    SNEVxVy(u8, u8),

    /** SNE Vx, byte - Skip next instruction if Vx != byte */
    SNE(u8, u8),

    /** LD Vx, byte */
    LD(u8, u8),

    /** RND Vx, byte & kk */
    RND(u8, u8),

    /** LD I, addr */
    LDI(u16),

    /** LD B, Vx - Load BCD value of Vx into I..I+2 */
    LDB(u8),

    /** Stores the value of register Vy in register Vx */
    LDVxFromVy(u8, u8),

    /** LD Vx, \[I\] - Read values from memory starting at location I into registers V0 through Vx. */
    LDVxFromI(u8),

    /** LD Vx, K - Wait for a key press, store the value of the key in Vx. */
    LDVxFromK(u8),

    /** LD \[I\] Vx - Copy the values of registers V0 through Vx into memory, starting at the address in I */
    LDIFromVx(u8),

    /** LD DT, Vx - Set delay timer = Vx */
    LDDTFromVx(u8),

    /** LD ST, Vx - Set sound timer = Vx */
    LDSTFromVx(u8),

    /** LD Vx, DT - Set Vx = delay timer */
    LDVxFromDT(u8),

    /** LD F, Vx - Set I = location of sprite for digit Vx */
    LDF(u8),

    /** DRW Vx, Vy, bytes - Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision */
    DRW(u8, u8, u8),

    /** ADD Vx, byte - Adds the value `kk` to the value of register Vx, then stores the result in V`x`. */
    ADD(u8, u8),

    // Non-standard, stops execution
    HLT,

    /** AND Vx, Vy - Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. */
    AND(u8, u8),

    /** ADD I, Vx - The values of I and Vx are added, and the results are stored in I. */
    ADDIVx(u8),

    /** SKP Vx, Skip next instruction if key with the value of Vx is pressed. */
    SKP(u8),

    /** SKNP Vx - Skip next instruction if key with the value of Vx is not pressed. */
    SKNP(u8),

    /** SUB Vx, Vy - Set Vx = Vx - Vy, set VF = NOT borrow */
    SUB(u8, u8),
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;
        match self {
            JP(arg0) => write!(f, "JP ({:03X})", arg0),
            CLS => write!(f, "CLS"),
            RET => write!(f, "RET"),
            CALL(arg0) => write!(f, "CALL ({:03X})", arg0),
            SE(arg0, arg1) => write!(f, "SE (V{:X}, {:02X})", arg0, arg1),
            SEVxVy(arg0, arg1) => write!(f, "SE (V{:X}, V{:X})", arg0, arg1),
            SNEVxVy(arg0, arg1) => write!(f, "SNE (V{:X}, V{:X})", arg0, arg1),
            SNE(arg0, arg1) => write!(f, "SNE (V{:X}, {:02X})", arg0, arg1),
            LD(arg0, arg1) => write!(f, "LD (V{:X}, {:02X})", arg0, arg1),
            RND(arg0, arg1) => write!(f, "RND (V{:X}, {:02X})", arg0, arg1),
            LDI(arg0) => write!(f, "LD (I, {:03X})", arg0),
            LDB(arg0) => write!(f, "LD (B, V{:X})", arg0),
            LDVxFromVy(arg0, arg1) => write!(f, "LD (V{:X}, V{:X})", arg0, arg1),
            LDVxFromI(arg0) => write!(f, "LD (V{:X}, I)", arg0),
            LDVxFromK(arg0) => write!(f, "LD (V{:X}, K)", arg0),
            LDIFromVx(arg0) => write!(f, "LD (I, V{:X})", arg0),
            LDDTFromVx(arg0) => write!(f, "LD (DT, V{:X})", arg0),
            LDSTFromVx(arg0) => write!(f, "LD (ST, V{:X})", arg0),
            LDVxFromDT(arg0) => write!(f, "LD (V{:X}, DT)", arg0),
            LDF(arg0) => write!(f, "LD (F, V{:X})", arg0),
            DRW(arg0, arg1, arg2) => write!(f, "DRW (V{:X}, V{:X}, {:X})", arg0, arg1, arg2),
            ADD(arg0, arg1) => write!(f, "ADD (V{:X}, V{:X})", arg0, arg1),
            HLT => write!(f, "HLT"),
            AND(arg0, arg1) => write!(f, "AND (V{:X}, V{:X})", arg0, arg1),
            ADDIVx(arg0) => write!(f, "ADD (I, V{:X})", arg0),
            SKP(arg0) => write!(f, "SKP (V{:X})", arg0),
            SKNP(arg0) => write!(f, "SKNP (V{:X})", arg0),
            SUB(arg0, arg1) => write!(f, "SUB (V{:X}, V{:X})", arg0, arg1),
        }
    }
}

fn parse_8_instruction(instruction: u16) -> InstructionParseResult {
    // Every instruction has the format 8xyN
    use InstructionParseResult::{Ok, Unparsed};
    let x = ((instruction & 0x0f00) >> 8) as u8;
    let y = ((instruction & 0x00f0) >> 4) as u8;
    let kind = (instruction & 0x000f) as u8;

    match kind {
        0 => Ok(Instruction::LDVxFromVy(x, y)),
        2 => Ok(Instruction::AND(x, y)),
        5 => Ok(Instruction::SUB(x, y)),
        _ => Unparsed,
    }
}

fn parse_f_instruction(instruction: u16) -> InstructionParseResult {
    use InstructionParseResult::{Ok, Unparsed};

    // We can give for granted that the instruction starts with 0xFnnn
    let register = ((instruction & 0x0f00) >> 8) as u8;
    let least_significant_byte = (instruction & 0x00ff) as u8;

    if least_significant_byte == 0x07 {
        // 0xFx07, LD Vx, DT
        return Ok(Instruction::LDVxFromDT(register));
    }

    if least_significant_byte == 0x0A {
        // 0xFx0A, LD Vx, K
        return Ok(Instruction::LDVxFromK(register));
    }

    if least_significant_byte == 0x15 {
        // 0xFx15, LD DT, Vx
        return Ok(Instruction::LDDTFromVx(register));
    }

    if least_significant_byte == 0x18 {
        // 0xFx18, LD ST, Vx
        return Ok(Instruction::LDSTFromVx(register));
    }

    if least_significant_byte == 0x1E {
        // 0xFx1E, ADD I, Vx
        return Ok(Instruction::ADDIVx(register));
    }

    if least_significant_byte == 0x29 {
        // 0xFx29: LD F, Vx
        return Ok(Instruction::LDF(register));
    }

    if least_significant_byte == 0x33 {
        // 0xFx33: LD B, Vx
        return Ok(Instruction::LDB(register));
    }

    if least_significant_byte == 0x65 {
        // 0xFx65 - LD Vx, [I]
        return Ok(Instruction::LDVxFromI(register));
    }
    if least_significant_byte == 0x55 {
        // 0xFx55 - LD [I], Vx
        return Ok(Instruction::LDIFromVx(register));
    }

    Unparsed
}

pub enum InstructionParseResult {
    Ok(Instruction),
    Unparsed,
}

pub fn parse_instruction(instruction: u16) -> InstructionParseResult {
    use InstructionParseResult::{Ok, Unparsed};

    // Parse "whole" instructions
    if instruction == 0x00E0 {
        return Ok(Instruction::CLS);
    }
    if instruction == 0x00EE {
        return Ok(Instruction::RET);
    }

    // Parse other instructions

    let most_significant_byte = (instruction & 0xff00) >> 8;
    let least_significant_byte = instruction & 0x00ff;

    let most_significant_byte: u8 = most_significant_byte.try_into().unwrap();
    let least_significant_byte: u8 = least_significant_byte.try_into().unwrap();

    return match most_significant_byte & 0xf0 {
        0x10 => {
            // 0x1nnn = JP nnn
            Ok(Instruction::JP(instruction & 0xfff))
        }
        0x20 => {
            // 0x2nnn = CALL nnn
            Ok(Instruction::CALL(instruction & 0xfff))
        }
        0x30 => {
            // 0x3xkk = SE Vx, kk
            let register_index = most_significant_byte & 0x0f;
            let value = least_significant_byte;
            Ok(Instruction::SE(register_index, value))
        }
        0x40 => {
            // 0x4xkk = SNE Vx, kk
            let register_index = most_significant_byte & 0x0f;
            let value = least_significant_byte;
            Ok(Instruction::SNE(register_index, value))
        }
        0x50 => {
            //0x5xy0 = SE Vx, Vy
            let x = ((instruction & 0x0f00) >> 8) as u8;
            let y = ((instruction & 0x00f0) >> 4) as u8;
            Ok(Instruction::SEVxVy(x, y))
        }
        0x60 => {
            // 0x6xkk = LD Vx, kk
            let register_index = most_significant_byte & 0x0f;
            let value = least_significant_byte;
            Ok(Instruction::LD(register_index, value))
        }
        0x70 => {
            // 0x7xkk = LD Vx, kk
            let register_index = most_significant_byte & 0x0f;
            let value = least_significant_byte;
            Ok(Instruction::ADD(register_index, value))
        }
        0x80 => {
            // 0x8xyN, needs more parsing
            parse_8_instruction(instruction)
        }
        0x90 => {
            //0x9xy0 = SNE Vx, Vy
            let x = ((instruction & 0x0f00) >> 8) as u8;
            let y = ((instruction & 0x00f0) >> 4) as u8;
            Ok(Instruction::SNEVxVy(x, y))
        }
        0xA0 => {
            // 0xAnnn = LD I, nnn
            let address = instruction & 0xfff;
            Ok(Instruction::LDI(address))
        }
        0xC0 => {
            // 0xCxkk = RND Vx, byte & kk
            let register_index = most_significant_byte & 0x0f;
            let bit_mask = least_significant_byte;
            Ok(Instruction::RND(register_index, bit_mask))
        }
        0xD0 => {
            // 0xDxyn = DRW Vx, Vy, sprite length
            let sprite_length = least_significant_byte & 0x0f;
            let x = most_significant_byte & 0x0f;
            let y = least_significant_byte & 0xf0 >> 4;
            Ok(Instruction::DRW(x, y, sprite_length))
        }
        0xE0 => {
            // 0xExnn - Skip if key is pressed/not pressed
            if least_significant_byte == 0x9E {
                Ok(Instruction::SKP(most_significant_byte & 0x0f))
            } else if least_significant_byte == 0xA1 {
                Ok(Instruction::SKNP(most_significant_byte & 0x0f))
            } else {
                Unparsed
            }
        }
        0xF0 => {
            // 0xFnnn, needs more parsing
            parse_f_instruction(instruction)
        }
        _ => Unparsed,
    };
}
