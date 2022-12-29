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

    // Non-standard, stops execution
    HLT,
}

fn parse_f_instruction(instruction: u16) -> Result<Instruction, ()> {
    // We can give for granted that the instruction starts with 0xFnnn
    let register = (instruction & 0x0f00) >> 8;
    let least_significant_byte = instruction & 0x00ff;

    if least_significant_byte == 0x33 {
        return Ok(Instruction::LDB(register as u8));
    }

    Err(())
}

pub fn parse_instruction(instruction: u16) -> Result<Instruction, ()> {
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
    if most_significant_byte & 0xf0 == 0xF0 {
        // 0xFnnn, needs more parsing
        return parse_f_instruction(instruction);
    }

    Err(())
}
