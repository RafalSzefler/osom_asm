#![allow(clippy::used_underscore_items)]
use osom_encoders_x86_64::encoders as enc;
use osom_encoders_x86_64::models as enc_models;

use super::fragment::Fragment;
use crate::assembler::implementation::PatchableImm32Instruction;
use crate::assembler::implementation::instructions;
use crate::{
    assembler::EmitError,
    models::{Immediate, Instruction, Size},
};

use super::X86_64Assembler;

mod const_encodings {
    pub const RET: &[u8] = super::enc::ret::encode_ret().as_slice();
}

impl X86_64Assembler {
    #[allow(clippy::needless_pass_by_value, clippy::unnecessary_wraps)]
    #[inline(always)]
    pub(crate) fn _emit_encoded_instruction(
        &mut self,
        encoded_instruction: enc_models::EncodedX86_64Instruction,
    ) -> Result<(), EmitError> {
        self._write_bytes_internal(encoded_instruction.as_slice());
        Ok(())
    }

    #[allow(clippy::unnecessary_wraps)]
    #[inline(always)]
    pub(crate) fn _emit_bytes(&mut self, bytes: &[u8]) -> Result<(), EmitError> {
        self._write_bytes_internal(bytes);
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn _emit_instruction(&mut self, instruction: &Instruction) -> Result<(), EmitError> {
        match instruction {
            // Pseudo-instructions
            Instruction::SetPrivate_Label { label } => {
                self._insert_label(*label)?;
                Ok(())
            }
            Instruction::SetPublic_Label { label } => {
                self._insert_label(*label)?;
                self.public_labels.push(*label);
                Ok(())
            }

            // The only two special instructions that have different encoding
            // depending on the context. Handled through fragmentation.
            Instruction::Jump_Label { dst } => {
                let new_fragment = Fragment::Relaxable_Jump {
                    variant: self._relaxation_variant(),
                    label: *dst,
                };
                self._push_new_fragment(new_fragment);
                Ok(())
            }
            Instruction::CondJump_Label { condition, dst } => {
                let new_fragment = Fragment::Relaxable_CondJump {
                    variant: self._relaxation_variant(),
                    condition: *condition,
                    label: *dst,
                };
                self._push_new_fragment(new_fragment);
                Ok(())
            }

            // Remaining instructions are not relaxable. But we still need to track
            // labels, since some instructions may utilize them, e.g. those that
            // use memory operands.
            Instruction::Ret => self._emit_bytes(const_encodings::RET),
            Instruction::Nop { length } => instructions::emit_nop_with_length(self, *length),
            Instruction::Mov_RegImm64 { dst, src } => instructions::emit_mov_reg_imm64(self, *dst, *src),
            Instruction::Mov_RegImm { dst, src } => instructions::emit_mov_reg_imm(self, *dst, *src),
            Instruction::Mov_MemImm { dst, src } => todo!(),
            Instruction::Mov_RegReg { dst, src } => instructions::emit_mov_reg_reg(self, *dst, *src),
            Instruction::Mov_MemReg { dst, src } => todo!(),
            Instruction::Mov_RegMem { dst, src } => instructions::emit_mov_reg_mem(self, *dst, src),
            Instruction::Add_RegImm { dst, src } => todo!(),
            Instruction::Add_MemImm { dst, src } => todo!(),
            Instruction::Add_RegReg { dst, src } => todo!(),
            Instruction::Add_MemReg { dst, src } => todo!(),
            Instruction::Add_RegMem { dst, src } => todo!(),
            Instruction::Sub_RegImm { dst, src } => todo!(),
            Instruction::Sub_MemImm { dst, src } => todo!(),
            Instruction::Sub_RegReg { dst, src } => todo!(),
            Instruction::Sub_MemReg { dst, src } => todo!(),
            Instruction::Sub_RegMem { dst, src } => todo!(),
            Instruction::Xor_RegImm { dst, src } => todo!(),
            Instruction::Xor_MemImm { dst, src } => todo!(),
            Instruction::Xor_RegReg { dst, src } => instructions::emit_xor_reg_reg(self, *dst, *src),
            Instruction::Xor_MemReg { dst, src } => todo!(),
            Instruction::Xor_RegMem { dst, src } => todo!(),
            Instruction::Jump_Reg { dst } => todo!(),
            Instruction::Jump_Mem { dst } => todo!(),
            Instruction::Call_Label { dst } => todo!(),
            Instruction::Call_Reg { dst } => todo!(),
            Instruction::Call_Mem { dst } => todo!(),
        }
    }
}
