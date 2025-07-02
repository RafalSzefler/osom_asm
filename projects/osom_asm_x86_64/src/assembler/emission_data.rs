use std::{collections::HashMap, mem::forget};

use crate::models::Label;

/// The classical Rust struct that represents the summary of the assembled code.
/// It allows the data to be mutated and moved around.
#[must_use]
pub struct DeconstructedEmissionData {
    pub emitted_bytes: i32,
    pub public_labels_positions: HashMap<Label, i32>,
}

/// The summary of the assembled code. This struct is immutable.
/// It can be decomposed into classical Rust struct through the
/// [`EmissionData::deconstruct`] method.
#[derive(Debug)]
#[must_use]
pub struct EmissionData {
    emitted_bytes: i32,
    public_labels_positions: HashMap<Label, i32>,
}

impl EmissionData {
    #[inline(always)]
    pub(crate) fn new(emitted_bytes: i32, public_labels_positions: HashMap<Label, i32>) -> Self {
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
            public_labels_positions: labels_to_position_map,
        }
    }

    /// Returns a map of public labels to their positions in the emitted code,
    /// relative to the beginning of the code, not to the passed stream.
    #[inline(always)]
    #[must_use]
    pub const fn public_labels_positions(&self) -> &HashMap<Label, i32> {
        &self.public_labels_positions
    }

    /// Returns the number of bytes emitted.
    #[inline(always)]
    #[must_use]
    pub const fn emitted_bytes(&self) -> i32 {
        self.emitted_bytes
    }
}
