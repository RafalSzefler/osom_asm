use crate::assembler::implementation::x86_64_assembler_assemble::{
    calculate_initial_offsets, calculate_labels_map, emit_fragments, patch_addresses,
    relax_instructions_and_update_offsets,
};
use crate::assembler::traits::X86_64Emitable;
use crate::assembler::{AssembleError, EmissionData, EmitError};

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

    /// Finalizes emitted code, optimizes it and writes the raw binary machine code back to the passed stream.
    pub fn assemble(mut self, stream: &mut impl std::io::Write) -> Result<EmissionData, AssembleError> {
        let mut offsets = calculate_initial_offsets(&self)?;
        relax_instructions_and_update_offsets(&mut self, &mut offsets)?;
        let labels_map = calculate_labels_map(&self, &offsets)?;
        patch_addresses(&mut self, &labels_map, &offsets)?;
        emit_fragments(&self, &labels_map, &offsets, stream)
    }
}
