use std::{collections::HashMap, mem::forget};

use crate::models::Label;

#[must_use]
pub struct DeconstructedEmissionData {
    pub emitted_bytes: usize,
    pub labels_to_position_map: HashMap<Label, usize>,
}

#[derive(Debug)]
#[must_use]
pub struct EmissionData {
    emitted_bytes: usize,
    public_labels_positions: HashMap<Label, usize>,
}

impl EmissionData {
    #[inline(always)]
    pub(crate) fn new(emitted_bytes: usize, public_labels_positions: HashMap<Label, usize>) -> Self {
        Self {
            emitted_bytes,
            public_labels_positions,
        }
    }

    /// Releases the internal data of the [`EmissionData`] into a free struct.
    #[inline(always)]
    pub const fn deconstruct(self) -> DeconstructedEmissionData {
        let emitted_bytes = self.emitted_bytes;
        let labels_to_position_map = unsafe { std::ptr::read(&self.public_labels_positions) };
        forget(self);
        DeconstructedEmissionData {
            emitted_bytes,
            labels_to_position_map,
        }
    }

    /// Returns a map of public labels to their positions in the emitted code,
    /// relative to the beginning of the code, not to the passed stream.
    #[inline(always)]
    #[must_use]
    pub const fn public_labels_positions(&self) -> &HashMap<Label, usize> {
        &self.public_labels_positions
    }

    /// Returns the number of bytes emitted.
    #[inline(always)]
    #[must_use]
    pub const fn emitted_bytes(&self) -> usize {
        self.emitted_bytes
    }
}
