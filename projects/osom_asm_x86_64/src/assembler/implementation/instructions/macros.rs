macro_rules! generate_fn_emit_reg_imm {
    ($name:ident) => {
        paste::paste! {
            pub fn [<emit_ $name _reg_imm>](asm: &mut crate::assembler::X86_64Assembler, dst: crate::models::GPR, src: crate::models::Immediate) -> Result<(), crate::assembler::EmitError> {
                use osom_encoders_x86_64::encoders as enc;
                use osom_encoders_x86_64::models as enc_models;
                use crate::models::{GPR, Size};

                unsafe {
                    let src_value = src.value();
                    let src_real_size = src.real_size();
                    if dst.eq(&GPR::AL) && src_real_size.eq(&Size::Bit8) {
                        let imm8 = enc_models::Immediate8::from_i8(src_value as i8);
                        asm._emit_encoded_instruction(enc::$name::[<encode_ $name _AL_imm8>](imm8))?;
                        return Ok(());
                    }

                    if dst.eq(&GPR::AX) && src_real_size.le(&Size::Bit16) {
                        let imm16 = enc_models::Immediate16::from_i16(src_value as i16);
                        asm._emit_encoded_instruction(enc::$name::[<encode_ $name _AX_imm16>](imm16))?;
                        return Ok(());
                    }

                    if dst.eq(&GPR::EAX) {
                        let imm32 = enc_models::Immediate32::from_i32(src_value);
                        asm._emit_encoded_instruction(enc::$name::[<encode_ $name _EAX_imm32>](imm32))?;
                        return Ok(());
                    }

                    if dst.eq(&GPR::RAX) {
                        let imm64 = enc_models::Immediate32::from_i32(src_value);
                        asm._emit_encoded_instruction(enc::$name::[<encode_ $name _RAX_imm32>](imm64))?;
                        return Ok(());
                    }

                    if dst.size() < src_real_size {
                        return Err(crate::assembler::EmitError::OperandSizeMismatch);
                    }

                    let dst_mem = dst.as_enc_gpr_or_mem();

                    match dst.size() {
                        Size::Bit8 => {
                            let imm8 = enc_models::Immediate8::from_i8(src_value as i8);
                            asm._emit_encoded_instruction(enc::$name::[<encode_ $name _rm8_imm8>](dst_mem, imm8))?;
                        }
                        Size::Bit16 => {
                            let imm16 = enc_models::Immediate16::from_i16(src_value as i16);
                            asm._emit_encoded_instruction(enc::$name::[<encode_ $name _rm16_imm16>](dst_mem, imm16))?;
                        }
                        Size::Bit32 => {
                            let imm32 = enc_models::Immediate32::from_i32(src_value);
                            asm._emit_encoded_instruction(enc::$name::[<encode_ $name _rm32_imm32>](dst_mem, imm32))?;
                        }
                        Size::Bit64 => {
                            let imm32 = enc_models::Immediate32::from_i32(src_value);
                            asm._emit_encoded_instruction(enc::$name::[<encode_ $name _rm64_imm32>](dst_mem, imm32))?;
                        }
                    }
                }
                Ok(())
            }
        }
    };
}

pub(crate) use generate_fn_emit_reg_imm;

macro_rules! generate_fn_emit_reg_reg {
    ($name:ident) => {
        paste::paste! {
            pub fn [<emit_ $name _reg_reg>](asm: &mut crate::assembler::X86_64Assembler, dst: crate::models::GPR, src: crate::models::GPR) -> Result<(), crate::assembler::EmitError> {
                use osom_encoders_x86_64::encoders as enc;
                use crate::models::Size;

                unsafe {
                    let size = dst.size();
                    if size != src.size() {
                        return Err(crate::assembler::EmitError::OperandSizeMismatch);
                    }
                    let dst_enc = dst.as_enc_gpr_or_mem();
                    let src_enc = src.as_enc_gpr();

                    match size {
                        Size::Bit8 => {
                            asm._emit_encoded_instruction(enc::$name::[<encode_ $name _rm8_reg8>](dst_enc, src_enc))?;
                        }
                        Size::Bit16 => {
                            asm._emit_encoded_instruction(enc::$name::[<encode_ $name _rm16_reg16>](dst_enc, src_enc))?;
                        }
                        Size::Bit32 => {
                            asm._emit_encoded_instruction(enc::$name::[<encode_ $name _rm32_reg32>](dst_enc, src_enc))?;
                        }
                        Size::Bit64 => {
                            asm._emit_encoded_instruction(enc::$name::[<encode_ $name _rm64_reg64>](dst_enc, src_enc))?;
                        }
                    }
                }
                Ok(())
            }
        }
    };
}

pub(crate) use generate_fn_emit_reg_reg;

macro_rules! generate_fn_emit_reg_mem {
    ($name:ident) => {
        paste::paste! {
            pub fn [<emit_ $name _reg_mem>](asm: &mut crate::assembler::X86_64Assembler, dst: crate::models::GPR, src: &crate::models::Memory) -> Result<(), crate::assembler::EmitError> {
                use osom_encoders_x86_64::encoders as enc;
                use osom_encoders_x86_64::models as enc_models;
                use crate::models::Size;
                use crate::assembler::implementation::instructions::helpers;

                unsafe {
                    let mem = src.as_enc_gpr_or_mem();
                    let dst_enc = dst.as_enc_gpr();

                    let instr = match dst.size() {
                        Size::Bit8 => enc::$name::[<encode_ $name _reg8_rm8>](dst_enc, mem),
                        Size::Bit16 => enc::$name::[<encode_ $name _reg16_rm16>](dst_enc, mem),
                        Size::Bit32 => enc::$name::[<encode_ $name _reg32_rm32>](dst_enc, mem),
                        Size::Bit64 => enc::$name::[<encode_ $name _reg64_rm64>](dst_enc, mem),
                    };

                    helpers::update_patchable_info(asm, src, &instr);

                    asm._emit_encoded_instruction(instr)?;
                }
                Ok(())
            }
        }
    };
}

pub(crate) use generate_fn_emit_reg_mem;

macro_rules! generate_fn_emit_mem_reg {
    ($name:ident) => {
        paste::paste! {
            pub fn [<emit_ $name _mem_reg>](asm: &mut crate::assembler::X86_64Assembler, dst: &crate::models::Memory, src: crate::models::GPR) -> Result<(), crate::assembler::EmitError> {
                use osom_encoders_x86_64::encoders as enc;
                use osom_encoders_x86_64::models as enc_models;
                use crate::models::Size;
                use crate::assembler::implementation::instructions::helpers;

                unsafe {
                    let mem = dst.as_enc_gpr_or_mem();
                    let src_enc = src.as_enc_gpr();

                    let instr = match src.size() {
                        Size::Bit8 => enc::$name::[<encode_ $name _rm8_reg8>](mem, src_enc),
                        Size::Bit16 => enc::$name::[<encode_ $name _rm16_reg16>](mem, src_enc),
                        Size::Bit32 => enc::$name::[<encode_ $name _rm32_reg32>](mem, src_enc),
                        Size::Bit64 => enc::$name::[<encode_ $name _rm64_reg64>](mem, src_enc),
                    };

                    helpers::update_patchable_info(asm, dst, &instr);

                    asm._emit_encoded_instruction(instr)?;
                }
                Ok(())
            }
        }
    };
}

pub(crate) use generate_fn_emit_mem_reg;

macro_rules! generate_fn_emit_mem_imm {
    ($name:ident) => {
        paste::paste! {
            pub fn [<emit_ $name _mem_imm>](asm: &mut crate::assembler::X86_64Assembler, dst: &crate::models::Memory, src: crate::models::Immediate) -> Result<(), crate::assembler::EmitError> {
                use osom_encoders_x86_64::encoders as enc;
                use osom_encoders_x86_64::models as enc_models;
                use crate::models::Size;
                use crate::assembler::implementation::instructions::helpers;

                unsafe {
                    let mem = dst.as_enc_gpr_or_mem();

                    let instr = match src.real_size() {
                        Size::Bit8 => {
                            let imm8 = enc_models::Immediate8::from_i8(src.value() as i8);
                            enc::$name::[<encode_ $name _rm8_imm8>](mem, imm8)
                        }
                        Size::Bit16 => {
                            let imm16 = enc_models::Immediate16::from_i16(src.value() as i16);
                            enc::$name::[<encode_ $name _rm16_imm16>](mem, imm16)
                        }
                        Size::Bit32 => {
                            let imm32 = enc_models::Immediate32::from_i32(src.value());
                            enc::$name::[<encode_ $name _rm32_imm32>](mem, imm32)
                        }
                        Size::Bit64 => {
                            let imm32 = enc_models::Immediate32::from_i32(src.value());
                            enc::$name::[<encode_ $name _rm64_imm32>](mem, imm32)
                        }
                    };

                    helpers::update_patchable_info_with_imm(asm, dst, &instr, src);
                    asm._emit_encoded_instruction(instr)?;
                }
                Ok(())
            }
        }
    };
}

pub(crate) use generate_fn_emit_mem_imm;

macro_rules! generate_group1_fn {
    ($name:ident) => {
        crate::assembler::implementation::instructions::macros::generate_fn_emit_reg_imm!($name);
        crate::assembler::implementation::instructions::macros::generate_fn_emit_reg_reg!($name);
        crate::assembler::implementation::instructions::macros::generate_fn_emit_reg_mem!($name);
        crate::assembler::implementation::instructions::macros::generate_fn_emit_mem_reg!($name);
        crate::assembler::implementation::instructions::macros::generate_fn_emit_mem_imm!($name);
    };
}

pub(crate) use generate_group1_fn;
