use osom_encoders_x86_64::models as enc_models;

use crate::assembler::X86_64Assembler;
use crate::assembler::implementation::PatchableImm32Instruction;
use crate::models::{Immediate, Memory, Size};

pub fn update_patchable_info(asm: &mut X86_64Assembler, src: &Memory, instr: &enc_models::EncodedX86_64Instruction) {
    if let Some(label) = src.get_label() {
        let position = asm._current_position();
        let instr_len = instr.as_slice().len() as u8;
        debug_assert!(instr_len >= 4, "Instruction length is too short");
        let patchable_instruction = PatchableImm32Instruction {
            instruction_position: position,
            instruction_length: instr_len,
            imm32_offset: instr_len - 4,
        };
        asm._push_patchable_instruction(*label, patchable_instruction);
    }
}

pub fn update_patchable_info_with_imm(
    asm: &mut X86_64Assembler,
    src: &Memory,
    instr: &enc_models::EncodedX86_64Instruction,
    imm: Immediate,
) {
    if let Some(label) = src.get_label() {
        let position = asm._current_position();
        let instr_len = instr.as_slice().len() as u8;
        let offset = match imm.real_size() {
            Size::Bit8 => 1,
            Size::Bit16 => 2,
            Size::Bit32 => 4,
            Size::Bit64 => 8,
        };
        let final_offset = 4 + offset;
        debug_assert!(instr_len >= final_offset, "Instruction length is too short");

        let patchable_instruction = PatchableImm32Instruction {
            instruction_position: position,
            instruction_length: instr_len,
            imm32_offset: instr_len - final_offset,
        };
        asm._push_patchable_instruction(*label, patchable_instruction);
    }
}
