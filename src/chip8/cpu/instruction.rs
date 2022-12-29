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

    // Non-standard, stops execution
    HLT,
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
        // 0x6xkk= = LD Vx, kk
        let register_index = most_significant_byte & 0x0f;
        let value = least_significant_byte;
        return Ok(Instruction::LD(register_index, value));
    }

    Err(())
}
