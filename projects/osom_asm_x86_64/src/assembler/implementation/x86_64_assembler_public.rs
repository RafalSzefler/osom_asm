use crate::assembler::EmitError;
use crate::assembler::traits::X86_64Emitable;

use super::X86_64Assembler;

impl X86_64Assembler {
    /// Emits the given value to the underlying [`X86_64Assembler`].
    ///
    /// The method accepts the private `X86_64Emitable` trait. At the moment
    /// the following types implement it: arrays, slices and [`Instruction`][`crate::models::Instruction`].
    #[allow(private_bounds)]
    pub fn emit(&mut self, value: impl X86_64Emitable) -> Result<(), EmitError> {
        value.emit_to(self)
    }
}
