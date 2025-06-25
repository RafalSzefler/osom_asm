use osom_encoders_x86_64::models as enc_models;
use osom_encoders_x86_64::encoders as enc;

use super::fragment::Fragment;
use crate::{assembler::EmitError, models::{Instruction, Immediate, Size}};

use super::X86_64Assembler;

impl X86_64Assembler {
    #[inline(always)]
    pub(crate) fn _emit_encoded_instruction(&mut self, encoded_instruction: enc_models::EncodedX86_64Instruction) -> Result<(), EmitError> {
        self._write_bytes_internal(encoded_instruction.as_slice())
    }

    #[inline(always)]
    pub(crate) fn _emit_bytes(&mut self, bytes: &[u8]) -> Result<(), EmitError> {
        self._write_bytes_internal(bytes)
    }

    pub(crate) fn _emit_instruction(&mut self, instruction: &Instruction) -> Result<(), EmitError> {
        unsafe {
            match instruction {
                // The only two special instructions that have different encoding
                // depending on the context. Handled through fragmentation.
                Instruction::Jump_Label { dst } => {
                    let new_fragment = Fragment::Relaxable_Jump { variant: self.relaxation_variant(), label: dst.clone() };
                    self.push_new_fragment(new_fragment);
                    Ok(())
                }
                Instruction::CondJump_Label { condition, dst } => {
                    let new_fragment = Fragment::Relaxable_CondJump { variant: self.relaxation_variant(), condition: condition.clone(), label: dst.clone() };
                    self.push_new_fragment(new_fragment);
                    Ok(())
                }

                // Remaining instructions are not relaxable. But we still need to track
                // labels.
                Instruction::Ret => self._emit_encoded_instruction(enc::ret::encode_ret()),
                Instruction::Nop { length } => {
                    let value = length.get();
                    let max_iter = value / 9;
                    for _ in 0..max_iter {
                        self._emit_encoded_instruction(enc::miscellaneous::encode_nop_with_length(9))?;
                    }
                    let remainder = (value % 9) as u8;
                    if remainder > 0 {
                        self._emit_encoded_instruction(enc::miscellaneous::encode_nop_with_length(remainder))?;
                    }
                    Ok(())
                }
                Instruction::Mov_RegImm64 { dst, src } => {
                    let src_value = src.value();
                    if src.real_size() == Size::Bit64 {
                        if dst.size() != Size::Bit64 {
                            return Err(EmitError::OperandSizeMismatch);
                        }
                        let imm64 = enc_models::Immediate64::from_i64(src_value);
                        self._emit_encoded_instruction(enc::mov::encode_mov_reg64_imm64(dst.as_enc_gpr(), imm64))?;
                        return Ok(());
                    }

                    let imm = Immediate::from(src_value as i32);
                    return self._emit_instruction(&Instruction::Mov_RegImm { dst: dst.clone(), src: imm });
                }
                Instruction::Mov_RegImm { dst, src } => {
                    if src.value() == 0 {
                        return self._emit_instruction(&Instruction::Xor_RegReg { dst: dst.clone(), src: dst.clone() });
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
                            self._emit_encoded_instruction(enc::mov::encode_mov_reg8_imm8(dst.as_enc_gpr(), imm8))?;
                        },
                        Size::Bit16 => {
                            let imm16 = enc_models::Immediate16::from_i16(src_value as i16);
                            self._emit_encoded_instruction(enc::mov::encode_mov_reg16_imm16(dst.as_enc_gpr(), imm16))?;
                        },
                        Size::Bit32 => {
                            let imm32 = enc_models::Immediate32::from_i32(src_value);
                            self._emit_encoded_instruction(enc::mov::encode_mov_reg32_imm32(dst.as_enc_gpr(), imm32))?;
                        },
                        Size::Bit64 => {
                            let imm32 = enc_models::Immediate32::from_i32(src_value);
                            self._emit_encoded_instruction(enc::mov::encode_mov_rm64_imm32(dst.as_enc_mem(), imm32))?;
                        },
                    };

                    Ok(())
                },
                Instruction::Mov_MemImm { dst, src } => todo!(),
                Instruction::Mov_RegReg { dst, src } => todo!(),
                Instruction::Mov_MemReg { dst, src } => todo!(),
                Instruction::Mov_RegMem { dst, src } => todo!(),
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
                Instruction::Xor_RegReg { dst, src } => {
                    let src_size = src.size();
                    let dst_size = dst.size();
                    if src_size != dst_size {
                        return Err(EmitError::OperandSizeMismatch);
                    }

                    let dst = dst.as_enc_gpr();
                    let src = src.as_enc_mem();

                    match src_size {
                        Size::Bit8 => {
                            self._emit_encoded_instruction(enc::xor::encode_xor_reg8_rm8(dst, src))?;
                        },
                        Size::Bit16 => {
                            self._emit_encoded_instruction(enc::xor::encode_xor_reg16_rm16(dst, src))?;
                        },
                        Size::Bit32 => {
                            self._emit_encoded_instruction(enc::xor::encode_xor_reg32_rm32(dst, src))?;
                        },
                        Size::Bit64 => {
                            self._emit_encoded_instruction(enc::xor::encode_xor_reg64_rm64(dst, src))?;
                        }
                    };

                    Ok(())
                }
                Instruction::Xor_MemReg { dst, src } => todo!(),
                Instruction::Xor_RegMem { dst, src } => todo!(),
                Instruction::SetPrivate_Label { label } => {
                    self.insert_label(label.clone())?;
                    Ok(())
                }
                Instruction::SetPublic_Label { label } => {
                    self.insert_label(label.clone())?;
                    self.public_labels.push(label.clone()); 
                    Ok(())
                }
                Instruction::Jump_Reg { dst } => todo!(),
                Instruction::Jump_Mem { dst } => todo!(),
                Instruction::Call_Label { dst } => todo!(),
                Instruction::Call_Reg { dst } => todo!(),
                Instruction::Call_Mem { dst } => todo!(),
            }
        }
    }
}
