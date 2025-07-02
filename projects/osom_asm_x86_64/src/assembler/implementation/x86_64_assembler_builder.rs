use std::collections::HashMap;

use crate::{
    assembler::implementation::{FragmentRelativePosition, fragment::FragmentOrderId},
    models::Label,
};

use super::X86_64Assembler;

/// Builder for the [`X86_64Assembler`].
#[must_use]
pub struct X86_64AssemblerBuilder {
    with_relaxation: bool,
    predefined_labels: Option<HashMap<Label, FragmentRelativePosition>>,
}

impl X86_64AssemblerBuilder {
    /// Creates a new [`X86_64AssemblerBuilder`] with the default settings.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            with_relaxation: true,
            predefined_labels: None,
        }
    }

    /// Toggles the relaxation algorithm for the underlying [`X86_64Assembler`].
    ///
    /// The relexation algorithm will emit short versions of jump instructions whenever
    /// it is possible. However the algorithm is quadratic in the worst case. Measuring
    /// performance is advised.
    #[inline(always)]
    pub const fn with_relaxation(mut self, with_relaxation: bool) -> Self {
        self.with_relaxation = with_relaxation;
        self
    }

    /// Sets the predefined labels for the underlying [`X86_64Assembler`].
    ///
    /// The predefined labels are used to emit jump instructions to the given labels.
    /// The offset is relative to the beginning of the code that is supposed to be emitted.
    ///
    /// By predefining labels we allow the newly generated code to jump to labels outside
    /// of the code itself.
    pub fn with_predefined_labels(mut self, predefined_labels: &HashMap<Label, i32>) -> Self {
        let capacity = core::cmp::max(predefined_labels.len(), 16);
        let mut label_offsets = HashMap::with_capacity(capacity);
        for (label, offset) in predefined_labels {
            label_offsets.insert(
                *label,
                FragmentRelativePosition {
                    fragment_id: FragmentOrderId::from_index(0),
                    in_fragment_offset: *offset,
                },
            );
        }
        self.predefined_labels = Some(label_offsets);
        self
    }

    /// Builds the [`X86_64Assembler`] with the given settings.
    pub fn build(self) -> X86_64Assembler {
        let predefined_labels = if let Some(predefined_labels) = self.predefined_labels {
            predefined_labels
        } else {
            HashMap::with_capacity(16)
        };
        X86_64Assembler::new(self.with_relaxation, predefined_labels)
    }
}

impl Default for X86_64AssemblerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
