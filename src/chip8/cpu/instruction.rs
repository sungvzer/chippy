#[derive(Debug)]
pub enum Instruction {
    /** JP addr */
    JP(u16),

    /** CLS */
    CLS,

    /** RET */
    RET,

    /** LD Vx, byte */
    LD(u8, u8),

    /** RND Vx, byte & kk */
    RND(u8, u8),

    /** LD I, addr */
    LDI(u16),

    /** LD B, Vx - Load BCD value of Vx into I..I+2 */
    LDB(u8),

    /** LD Vx, \[I\] - Read values from memory starting at location I into registers V0 through Vx. */
    LDVxFromI(u8),

    /** LD Vx, K - Wait for a key press, store the value of the key in Vx. */
    LDVxFromK(u8),

    /** LD \[I\] Vx - Copy the values of registers V0 through Vx into memory, starting at the address in I */
    LDIFromVx(u8),

    /** LD F, Vx - Set I = location of sprite for digit Vx */
    LDF(u8),

    /** DRW Vx, Vy, bytes - Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision */
    DRW(u8, u8, u8),

    /** ADD Vx, byte - Adds the value `kk` to the value of register Vx, then stores the result in V`x`. */
    ADD(u8, u8),

    // Non-standard, stops execution
    HLT,
}

fn parse_f_instruction(instruction: u16) -> InstructionParseResult {
    use InstructionParseResult::{Ok, Unparsed};

    // We can give for granted that the instruction starts with 0xFnnn
    let register = ((instruction & 0x0f00) >> 8) as u8;
    let least_significant_byte = (instruction & 0x00ff) as u8;

    if least_significant_byte == 0x0A {
        // 0xFx0A, LD Vx, K
        return Ok(Instruction::LDVxFromK(register));
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

    if most_significant_byte & 0xf0 == 0x10 {
        // 0x1nnn = JP nnn
        return Ok(Instruction::JP(instruction & 0xfff));
    }
    if most_significant_byte & 0xf0 == 0x60 {
        // 0x6xkk = LD Vx, kk
        let register_index = most_significant_byte & 0x0f;
        let value = least_significant_byte;
        return Ok(Instruction::LD(register_index, value));
    }

    if most_significant_byte & 0xf0 == 0x70 {
        // 0x7xkk = LD Vx, kk
        let register_index = most_significant_byte & 0x0f;
        let value = least_significant_byte;
        return Ok(Instruction::ADD(register_index, value));
    }

    if most_significant_byte & 0xf0 == 0xA0 {
        // 0xAnnn = LD I, nnn
        let address = instruction & 0xfff;
        return Ok(Instruction::LDI(address));
    }

    if most_significant_byte & 0xf0 == 0xC0 {
        // 0xCxkk = RND Vx, byte & kk
        let register_index = most_significant_byte & 0x0f;
        let bit_mask = least_significant_byte;
        return Ok(Instruction::RND(register_index, bit_mask));
    }

    if most_significant_byte & 0xf0 == 0xD0 {
        // 0xDxyn = DRW Vx, Vy, sprite length
        let sprite_length = least_significant_byte & 0x0f;
        let x = most_significant_byte & 0x0f;
        let y = least_significant_byte & 0xf0 >> 4;
        return Ok(Instruction::DRW(x, y, sprite_length));
    }

    if most_significant_byte & 0xf0 == 0xF0 {
        // 0xFnnn, needs more parsing
        return parse_f_instruction(instruction);
    }

    Unparsed
}
