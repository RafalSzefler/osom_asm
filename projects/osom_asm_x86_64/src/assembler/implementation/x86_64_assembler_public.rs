use crate::assembler::{EmitError, X86_64Emitable};

use super::X86_64Assembler;

impl X86_64Assembler {
    #[allow(private_bounds)]
    pub fn emit<T: X86_64Emitable>(&mut self, value: T) -> Result<(), EmitError> {
        value.emit_to(self)
    }
}
