use std::num::NonZero;

use osom_encoders_x86_64::encoders as enc;

use crate::assembler::{EmitError, X86_64Assembler};

pub fn emit_nop_with_length(asm: &mut X86_64Assembler, length: NonZero<u32>) -> Result<(), EmitError> {
    let value = length.get();
    let max_iter = value / 9;
    if max_iter > 0 {
        let maxed_instruction = enc::miscellaneous::encode_nop_with_length(9);
        let maxed_instruction_slice = maxed_instruction.as_slice();
        for _ in 0..max_iter {
            asm._emit_bytes(maxed_instruction_slice)?;
        }
    }
    let remainder = (value % 9) as u8;
    if remainder > 0 {
        asm._emit_encoded_instruction(enc::miscellaneous::encode_nop_with_length(remainder))?;
    }
    Ok(())
}
