#![allow(clippy::used_underscore_items)]

use osom_encoders_x86_64::models::EncodedX86_64Instruction;

use crate::{assembler::X86_64Assembler, models::Instruction};

use super::EmitError;

pub(crate) trait X86_64Emitable {
    fn emit_to(self, assembler: &mut X86_64Assembler) -> Result<(), EmitError>;
}

impl X86_64Emitable for &[u8] {
    fn emit_to(self, assembler: &mut X86_64Assembler) -> Result<(), EmitError> {
        assembler._emit_bytes(self)
    }
}

impl<const N: usize> X86_64Emitable for [u8; N] {
    fn emit_to(self, assembler: &mut X86_64Assembler) -> Result<(), EmitError> {
        assembler._emit_bytes(&self)
    }
}

impl<const N: usize> X86_64Emitable for &[u8; N] {
    fn emit_to(self, assembler: &mut X86_64Assembler) -> Result<(), EmitError> {
        assembler._emit_bytes(self)
    }
}

impl X86_64Emitable for Instruction {
    fn emit_to(self, assembler: &mut X86_64Assembler) -> Result<(), EmitError> {
        assembler._emit_instruction(&self)
    }
}

impl X86_64Emitable for EncodedX86_64Instruction {
    fn emit_to(self, assembler: &mut X86_64Assembler) -> Result<(), EmitError> {
        assembler._emit_encoded_instruction(self)
    }
}
