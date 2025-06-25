use osom_encoders_x86_64::encoders as enc;
use osom_encoders_x86_64::models as enc_models;

use crate::assembler::implementation::{PatchableImm32Instruction, instructions::emit_xor_reg_reg};
use crate::assembler::{EmitError, X86_64Assembler};
use crate::models::{GPR, Immediate, Immediate64, Memory, Size};

pub fn emit_mov_reg_imm64(asm: &mut X86_64Assembler, dst: GPR, src: Immediate64) -> Result<(), EmitError> {
    unsafe {
        let src_value = src.value();
        if src.real_size() == Size::Bit64 {
            if dst.size() != Size::Bit64 {
                return Err(EmitError::OperandSizeMismatch);
            }
            let imm64 = enc_models::Immediate64::from_i64(src_value);
            asm._emit_encoded_instruction(enc::mov::encode_mov_reg64_imm64(dst.as_enc_gpr(), imm64))?;
            return Ok(());
        }

        let imm = Immediate::from(src_value as i32);
        emit_mov_reg_imm(asm, dst, imm)
    }
}

pub fn emit_mov_reg_imm(asm: &mut X86_64Assembler, dst: GPR, src: Immediate) -> Result<(), EmitError> {
    unsafe {
        if dst.size() >= Size::Bit32 && src.value() == 0 {
            return emit_xor_reg_reg(asm, dst, dst);
        }

        let dst_size = dst.size();
        let src_size = src.real_size();
        if dst_size < src_size {
            return Err(EmitError::OperandSizeMismatch);
        }

        let src_value = src.value();
        match dst_size {
            Size::Bit8 => {
                let imm8 = enc_models::Immediate8::from_i8(src_value as i8);
                asm._emit_encoded_instruction(enc::mov::encode_mov_reg8_imm8(dst.as_enc_gpr(), imm8))?;
            }
            Size::Bit16 => {
                let imm16 = enc_models::Immediate16::from_i16(src_value as i16);
                asm._emit_encoded_instruction(enc::mov::encode_mov_reg16_imm16(dst.as_enc_gpr(), imm16))?;
            }
            Size::Bit32 => {
                let imm32 = enc_models::Immediate32::from_i32(src_value);
                asm._emit_encoded_instruction(enc::mov::encode_mov_reg32_imm32(dst.as_enc_gpr(), imm32))?;
            }
            Size::Bit64 => {
                let imm32 = enc_models::Immediate32::from_i32(src_value);
                asm._emit_encoded_instruction(enc::mov::encode_mov_rm64_imm32(dst.as_enc_mem(), imm32))?;
            }
        }
    }
    Ok(())
}

pub fn emit_mov_reg_reg(asm: &mut X86_64Assembler, dst: GPR, src: GPR) -> Result<(), EmitError> {
    unsafe {
        let size = src.size();
        if dst.size() != size {
            return Err(EmitError::OperandSizeMismatch);
        }

        match size {
            Size::Bit8 => {
                asm._emit_encoded_instruction(enc::mov::encode_mov_reg8_rm8(dst.as_enc_gpr(), src.as_enc_mem()))?;
            }
            Size::Bit16 => {
                asm._emit_encoded_instruction(enc::mov::encode_mov_reg16_rm16(dst.as_enc_gpr(), src.as_enc_mem()))?;
            }
            Size::Bit32 => {
                asm._emit_encoded_instruction(enc::mov::encode_mov_reg32_rm32(dst.as_enc_gpr(), src.as_enc_mem()))?;
            }
            Size::Bit64 => {
                asm._emit_encoded_instruction(enc::mov::encode_mov_reg64_rm64(dst.as_enc_gpr(), src.as_enc_mem()))?;
            }
        }
    }
    Ok(())
}

pub fn emit_mov_reg_mem(asm: &mut X86_64Assembler, dst: GPR, src: &Memory) -> Result<(), EmitError> {
    unsafe {
        let mem = src.as_enc_mem();
        let mem = enc_models::GPROrMemory::Memory { memory: mem };

        let instr = match dst.size() {
            Size::Bit8 => enc::mov::encode_mov_reg8_rm8(dst.as_enc_gpr(), mem),
            Size::Bit16 => enc::mov::encode_mov_reg16_rm16(dst.as_enc_gpr(), mem),
            Size::Bit32 => enc::mov::encode_mov_reg32_rm32(dst.as_enc_gpr(), mem),
            Size::Bit64 => enc::mov::encode_mov_reg64_rm64(dst.as_enc_gpr(), mem),
        };

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

        asm._emit_encoded_instruction(instr)?;
    }
    Ok(())
}
