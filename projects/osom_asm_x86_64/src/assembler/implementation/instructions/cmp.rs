use osom_encoders_x86_64::encoders as enc;
use osom_encoders_x86_64::models as enc_models;

use crate::assembler::implementation::{PatchableImm32Instruction, instructions::emit_xor_reg_reg};
use crate::assembler::{EmitError, X86_64Assembler};
use crate::models::{GPR, Immediate, Immediate64, Memory, Size};

use super::helpers;

pub fn emit_cmp_reg_imm(asm: &mut X86_64Assembler, dst: GPR, src: Immediate) -> Result<(), EmitError> {
    unsafe {
        let src_value = src.value();
        let src_real_size = src.real_size();
        if dst == GPR::AL && src_real_size == Size::Bit8 {
            let imm8 = enc_models::Immediate8::from_i8(src_value as i8);
            asm._emit_encoded_instruction(enc::cmp::encode_cmp_AL_imm8(imm8))?;
            return Ok(());
        }

        if dst == GPR::AX && src_real_size <= Size::Bit16 {
            let imm16 = enc_models::Immediate16::from_i16(src_value as i16);
            asm._emit_encoded_instruction(enc::cmp::encode_cmp_AX_imm16(imm16))?;
            return Ok(());
        }

        if dst == GPR::EAX {
            let imm32 = enc_models::Immediate32::from_i32(src_value);
            asm._emit_encoded_instruction(enc::cmp::encode_cmp_EAX_imm32(imm32))?;
            return Ok(());
        }

        if dst == GPR::RAX {
            let imm64 = enc_models::Immediate32::from_i32(src_value);
            asm._emit_encoded_instruction(enc::cmp::encode_cmp_RAX_imm32(imm64))?;
            return Ok(());
        }

        if dst.size() < src_real_size {
            return Err(EmitError::OperandSizeMismatch);
        }

        match dst.size() {
            Size::Bit8 => {
                let imm8 = enc_models::Immediate8::from_i8(src_value as i8);
                asm._emit_encoded_instruction(enc::cmp::encode_cmp_rm8_imm8(dst.as_enc_mem(), imm8))?;
            }
            Size::Bit16 => {
                let imm16 = enc_models::Immediate16::from_i16(src_value as i16);
                asm._emit_encoded_instruction(enc::cmp::encode_cmp_rm16_imm16(dst.as_enc_mem(), imm16))?;
            }
            Size::Bit32 => {
                let imm32 = enc_models::Immediate32::from_i32(src_value);
                asm._emit_encoded_instruction(enc::cmp::encode_cmp_rm32_imm32(dst.as_enc_mem(), imm32))?;
            }
            Size::Bit64 => {
                let imm32 = enc_models::Immediate32::from_i32(src_value);
                asm._emit_encoded_instruction(enc::cmp::encode_cmp_rm64_imm32(dst.as_enc_mem(), imm32))?;
            }
        }
    }
    Ok(())
}

pub fn emit_cmp_reg_reg(asm: &mut X86_64Assembler, dst: GPR, src: GPR) -> Result<(), EmitError> {
    unsafe {
        let size = dst.size();
        if size != src.size() {
            return Err(EmitError::OperandSizeMismatch);
        }

        match size {
            Size::Bit8 => {
                asm._emit_encoded_instruction(enc::cmp::encode_cmp_rm8_reg8(dst.as_enc_mem(), src.as_enc_gpr()))?;
            }
            Size::Bit16 => {
                asm._emit_encoded_instruction(enc::cmp::encode_cmp_rm16_reg16(dst.as_enc_mem(), src.as_enc_gpr()))?;
            }
            Size::Bit32 => {
                asm._emit_encoded_instruction(enc::cmp::encode_cmp_rm32_reg32(dst.as_enc_mem(), src.as_enc_gpr()))?;
            }
            Size::Bit64 => {
                asm._emit_encoded_instruction(enc::cmp::encode_cmp_rm64_reg64(dst.as_enc_mem(), src.as_enc_gpr()))?;
            }
        }
    }
    Ok(())
}

pub fn emit_cmp_mem_imm(asm: &mut X86_64Assembler, dst: &Memory, src: Immediate) -> Result<(), EmitError> {
    unsafe {
        let mem = dst.as_enc_mem();
        let mem = enc_models::GPROrMemory::Memory { memory: mem };

        let instr = match src.real_size() {
            Size::Bit8 => {
                let imm8 = enc_models::Immediate8::from_i8(src.value() as i8);
                enc::cmp::encode_cmp_rm8_imm8(mem, imm8)
            }
            Size::Bit16 => {
                let imm16 = enc_models::Immediate16::from_i16(src.value() as i16);
                enc::cmp::encode_cmp_rm16_imm16(mem, imm16)
            }
            Size::Bit32 => {
                let imm32 = enc_models::Immediate32::from_i32(src.value());
                enc::cmp::encode_cmp_rm32_imm32(mem, imm32)
            }
            Size::Bit64 => {
                let imm32 = enc_models::Immediate32::from_i32(src.value());
                enc::cmp::encode_cmp_rm64_imm32(mem, imm32)
            }
        };

        helpers::update_patchable_info_with_imm(asm, dst, &instr, src);
        asm._emit_encoded_instruction(instr)?;
    }
    Ok(())
}

pub fn emit_cmp_reg_mem(asm: &mut X86_64Assembler, dst: GPR, src: &Memory) -> Result<(), EmitError> {
    unsafe {
        let mem = src.as_enc_mem();
        let mem = enc_models::GPROrMemory::Memory { memory: mem };

        let instr = match dst.size() {
            Size::Bit8 => enc::mov::encode_mov_reg8_rm8(dst.as_enc_gpr(), mem),
            Size::Bit16 => enc::mov::encode_mov_reg16_rm16(dst.as_enc_gpr(), mem),
            Size::Bit32 => enc::mov::encode_mov_reg32_rm32(dst.as_enc_gpr(), mem),
            Size::Bit64 => enc::mov::encode_mov_reg64_rm64(dst.as_enc_gpr(), mem),
        };

        helpers::update_patchable_info(asm, src, &instr);

        asm._emit_encoded_instruction(instr)?;
    }
    Ok(())
}

pub fn emit_cmp_mem_reg(asm: &mut X86_64Assembler, dst: &Memory, src: GPR) -> Result<(), EmitError> {
    unsafe {
        let mem = dst.as_enc_mem();
        let mem = enc_models::GPROrMemory::Memory { memory: mem };

        let instr = match src.size() {
            Size::Bit8 => enc::cmp::encode_cmp_rm8_reg8(mem, src.as_enc_gpr()),
            Size::Bit16 => enc::cmp::encode_cmp_rm16_reg16(mem, src.as_enc_gpr()),
            Size::Bit32 => enc::cmp::encode_cmp_rm32_reg32(mem, src.as_enc_gpr()),
            Size::Bit64 => enc::cmp::encode_cmp_rm64_reg64(mem, src.as_enc_gpr()),
        };

        helpers::update_patchable_info(asm, dst, &instr);

        asm._emit_encoded_instruction(instr)?;
    }
    Ok(())
}
