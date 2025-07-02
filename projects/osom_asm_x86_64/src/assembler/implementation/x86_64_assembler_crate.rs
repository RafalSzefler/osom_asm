use osom_encoders_x86_64::encoders as enc;
use osom_encoders_x86_64::models as enc_models;

use super::fragment::Fragment;
use crate::assembler::implementation::instructions;
use crate::{assembler::EmitError, models::Instruction};

use super::X86_64Assembler;

mod const_encodings {
    pub(super) const RET: &[u8] = super::enc::ret::encode_ret().as_slice();
    pub(super) const CPUID: &[u8] = super::enc::singleton::encode_cpuid().as_slice();
    pub(super) const SYSCALL: &[u8] = super::enc::singleton::encode_syscall().as_slice();
    pub(super) const LOCK: &[u8] = super::enc::singleton::encode_lock().as_slice();
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
            Instruction::SetPrivate_Label { label } => {
                self._insert_label(*label)?;
                Ok(())
            }
            Instruction::SetPublic_Label { label } => {
                self._insert_label(*label)?;
                self.public_labels.push(*label);
                Ok(())
            }
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
            Instruction::Ret => self._emit_bytes(const_encodings::RET),
            Instruction::Cpuid => self._emit_bytes(const_encodings::CPUID),
            Instruction::Nop { length } => instructions::emit_nop_with_length(self, *length),
            Instruction::Mov_RegImm64 { dst, src } => instructions::emit_mov_reg_imm64(self, *dst, *src),
            Instruction::Mov_RegImm { dst, src } => instructions::emit_mov_reg_imm(self, *dst, *src),
            Instruction::Mov_MemImm { dst, src } => instructions::emit_mov_mem_imm(self, dst, *src),
            Instruction::Mov_RegReg { dst, src } => instructions::emit_mov_reg_reg(self, *dst, *src),
            Instruction::Mov_MemReg { dst, src } => instructions::emit_mov_mem_reg(self, dst, *src),
            Instruction::Mov_RegMem { dst, src } => instructions::emit_mov_reg_mem(self, *dst, src),
            Instruction::Cmp_RegImm { dst, src } => instructions::emit_cmp_reg_imm(self, *dst, *src),
            Instruction::Cmp_RegReg { dst, src } => instructions::emit_cmp_reg_reg(self, *dst, *src),
            Instruction::Cmp_MemImm { dst, src } => instructions::emit_cmp_mem_imm(self, dst, *src),
            Instruction::Cmp_RegMem { dst, src } => instructions::emit_cmp_reg_mem(self, *dst, src),
            Instruction::Cmp_MemReg { dst, src } => instructions::emit_cmp_mem_reg(self, dst, *src),
            Instruction::Add_RegImm { dst, src } => instructions::emit_add_reg_imm(self, *dst, *src),
            Instruction::Add_MemImm { dst, src } => instructions::emit_add_mem_imm(self, dst, *src),
            Instruction::Add_RegReg { dst, src } => instructions::emit_add_reg_reg(self, *dst, *src),
            Instruction::Add_MemReg { dst, src } => instructions::emit_add_mem_reg(self, dst, *src),
            Instruction::Add_RegMem { dst, src } => instructions::emit_add_reg_mem(self, *dst, src),
            Instruction::Sub_RegImm { dst, src } => instructions::emit_sub_reg_imm(self, *dst, *src),
            Instruction::Sub_MemImm { dst, src } => instructions::emit_sub_mem_imm(self, dst, *src),
            Instruction::Sub_RegReg { dst, src } => instructions::emit_sub_reg_reg(self, *dst, *src),
            Instruction::Sub_MemReg { dst, src } => instructions::emit_sub_mem_reg(self, dst, *src),
            Instruction::Sub_RegMem { dst, src } => instructions::emit_sub_reg_mem(self, *dst, src),
            Instruction::Xor_RegImm { dst, src } => instructions::emit_xor_reg_imm(self, *dst, *src),
            Instruction::Xor_MemImm { dst, src } => instructions::emit_xor_mem_imm(self, dst, *src),
            Instruction::Xor_RegReg { dst, src } => instructions::emit_xor_reg_reg(self, *dst, *src),
            Instruction::Xor_MemReg { dst, src } => instructions::emit_xor_mem_reg(self, dst, *src),
            Instruction::Xor_RegMem { dst, src } => instructions::emit_xor_reg_mem(self, *dst, src),
            Instruction::Jump_Reg { dst } => instructions::emit_jmp_reg(self, *dst),
            Instruction::Jump_Mem { dst } => instructions::emit_jmp_mem(self, dst),
            Instruction::Call_Label { dst } => instructions::emit_call_label(self, *dst),
            Instruction::Call_Reg { dst } => instructions::emit_call_reg(self, *dst),
            Instruction::Call_Mem { dst } => instructions::emit_call_mem(self, dst),
            Instruction::Push_Imm { src } => instructions::emit_push_imm(self, *src),
            Instruction::Push_Reg { src } => instructions::emit_push_reg(self, *src),
            Instruction::Push_Mem { src } => instructions::emit_push_mem(self, src),
            Instruction::Pop_Reg { src } => instructions::emit_pop_reg(self, *src),
            Instruction::Pop_Mem { src } => instructions::emit_pop_mem(self, src),
            Instruction::Int_Imm { src } => instructions::emit_int_imm(self, *src),
            Instruction::Syscall => self._emit_bytes(const_encodings::SYSCALL),
            Instruction::Lock => self._emit_bytes(const_encodings::LOCK),
        }
    }
}
