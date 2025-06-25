use osom_encoders_x86_64::encoders as enc;
use osom_encoders_x86_64::models as enc_models;

use crate::assembler::{EmitError, X86_64Assembler};
use crate::models::{GPR, Size};

pub fn emit_xor_reg_reg(asm: &mut X86_64Assembler, dst: GPR, src: GPR) -> Result<(), EmitError> {
    unsafe {
        let src_size = src.size();
        let dst_size = dst.size();
        if src_size != dst_size {
            return Err(EmitError::OperandSizeMismatch);
        }

        let dst = dst.as_enc_gpr();
        let src = src.as_enc_mem();

        match src_size {
            Size::Bit8 => {
                asm._emit_encoded_instruction(enc::xor::encode_xor_reg8_rm8(dst, src))?;
            }
            Size::Bit16 => {
                asm._emit_encoded_instruction(enc::xor::encode_xor_reg16_rm16(dst, src))?;
            }
            Size::Bit32 => {
                asm._emit_encoded_instruction(enc::xor::encode_xor_reg32_rm32(dst, src))?;
            }
            Size::Bit64 => {
                asm._emit_encoded_instruction(enc::xor::encode_xor_reg64_rm64(dst, src))?;
            }
        }
    }

    Ok(())
}
