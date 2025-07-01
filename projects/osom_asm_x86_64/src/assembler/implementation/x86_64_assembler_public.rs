use crate::assembler::EmitError;
use crate::assembler::traits::X86_64Emitable;

use super::X86_64Assembler;

impl X86_64Assembler {
    #[allow(private_bounds)]
    pub fn emit(&mut self, value: impl X86_64Emitable) -> Result<(), EmitError> {
        value.emit_to(self)
    }
}
