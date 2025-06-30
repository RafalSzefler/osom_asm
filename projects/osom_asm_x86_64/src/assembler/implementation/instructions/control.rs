use osom_encoders_x86_64::encoders as enc;
use osom_encoders_x86_64::models as enc_models;

use crate::assembler::implementation::instructions::helpers::update_labeled_instruction;
use crate::{
    assembler::{EmitError, X86_64Assembler},
    models::{GPR, Label, Memory},
};

pub fn emit_jmp_reg(asm: &mut X86_64Assembler, dst: GPR) -> Result<(), EmitError> {
    unsafe { asm._emit_encoded_instruction(enc::jmp::encode_jmp_rm64(dst.as_enc_gpr_or_mem())) }
}

pub fn emit_jmp_mem(asm: &mut X86_64Assembler, dst: &Memory) -> Result<(), EmitError> {
    unsafe { asm._emit_encoded_instruction(enc::jmp::encode_jmp_rm64(dst.as_enc_gpr_or_mem())) }
}

pub fn emit_call_label(asm: &mut X86_64Assembler, dst: Label) -> Result<(), EmitError> {
    let instr = enc::call::encode_call_imm32(enc_models::Immediate32::from_i32(0));
    update_labeled_instruction(asm, dst, &instr);
    asm._emit_encoded_instruction(instr)
}

pub fn emit_call_reg(asm: &mut X86_64Assembler, dst: GPR) -> Result<(), EmitError> {
    unsafe { asm._emit_encoded_instruction(enc::call::encode_call_rm64(dst.as_enc_gpr_or_mem())) }
}

pub fn emit_call_mem(asm: &mut X86_64Assembler, dst: &Memory) -> Result<(), EmitError> {
    unsafe { asm._emit_encoded_instruction(enc::call::encode_call_rm64(dst.as_enc_gpr_or_mem())) }
}
