use osom_encoders_x86_64::encoders as enc;
use osom_encoders_x86_64::models as enc_models;

use crate::assembler::implementation::instructions::helpers::update_patchable_info;
use crate::{
    assembler::{EmitError, X86_64Assembler},
    models::{GPR, Immediate, Memory, Size},
};

pub fn emit_push_imm(asm: &mut X86_64Assembler, src: Immediate) -> Result<(), EmitError> {
    let instruction = match src.real_size() {
        Size::Bit32 => {
            let imm32 = enc_models::Immediate32::from_i32(src.value());
            enc::push::encode_push_imm32(imm32)
        }
        Size::Bit16 => {
            let imm16 = enc_models::Immediate16::from_i16(src.value() as i16);
            enc::push::encode_push_imm16(imm16)
        }
        Size::Bit8 => {
            let imm8 = enc_models::Immediate8::from_i8(src.value() as i8);
            enc::push::encode_push_imm8(imm8)
        }
        _ => unreachable!(),
    };

    asm._emit_encoded_instruction(instruction)
}

pub fn emit_push_reg(asm: &mut X86_64Assembler, src: GPR) -> Result<(), EmitError> {
    if src.size() != Size::Bit64 {
        return Err(EmitError::OperandSizeMismatch);
    }

    unsafe { asm._emit_encoded_instruction(enc::push::encode_push_reg64(src.as_enc_gpr())) }
}

pub fn emit_push_mem(asm: &mut X86_64Assembler, src: &Memory) -> Result<(), EmitError> {
    let instruction = unsafe { enc::push::encode_push_rm64(src.as_enc_gpr_or_mem()) };
    update_patchable_info(asm, src, &instruction);
    asm._emit_encoded_instruction(instruction)
}

pub fn emit_pop_reg(asm: &mut X86_64Assembler, src: GPR) -> Result<(), EmitError> {
    if src.size() != Size::Bit64 {
        return Err(EmitError::OperandSizeMismatch);
    }

    unsafe { asm._emit_encoded_instruction(enc::pop::encode_pop_reg64(src.as_enc_gpr())) }
}

pub fn emit_pop_mem(asm: &mut X86_64Assembler, src: &Memory) -> Result<(), EmitError> {
    let instruction = unsafe { enc::pop::encode_pop_rm64(src.as_enc_gpr_or_mem()) };
    update_patchable_info(asm, src, &instruction);
    asm._emit_encoded_instruction(instruction)
}
