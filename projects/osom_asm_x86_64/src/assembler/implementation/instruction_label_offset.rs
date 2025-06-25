use crate::models::{Immediate, Instruction, Size};

const fn byte_size_of_immediate(imm: &Immediate) -> i32 {
    match imm.real_size() {
        Size::Bit8 => 1,
        Size::Bit16 => 2,
        Size::Bit32 => 4,
        _ => panic!("Unexpected immediate size"),
    }
}

const fn immediate_offset(instruction: &Instruction) -> i32 {
    match instruction {
        Instruction::Mov_MemImm { dst: _, src } =>
            byte_size_of_immediate(src),
        Instruction::Add_MemImm { dst: _, src } =>
            byte_size_of_immediate(src),
        Instruction::Sub_MemImm { dst: _, src } =>
            byte_size_of_immediate(src),
        Instruction::Xor_MemImm { dst: _, src } =>
            byte_size_of_immediate(src),
        _ => 0,
    }
}

/// Returns the offset from the *end* of the instruction to the label.
/// Or rather: to the place where imm32 lives in RIP-relative memory address.
///
/// This applies only to instructions that involve Mem operand. Typically
/// this is -4 from the end of the instruction, unless the instruction
/// encodes immediate value, then the offset is -(4 + size of the immediate).
#[inline]
pub const fn get_instruction_label_offset(instruction: &Instruction) -> i32 {
    -(4 + immediate_offset(instruction))
}
