use std::{collections::HashMap, mem::forget};

use crate::models::Label;

pub struct DeconstructedEmissionData {
    pub emitted_bytes: usize,
    pub labels_to_position_map: HashMap<Label, usize>,
}

#[derive(Debug)]
pub struct EmissionData {
    emitted_bytes: usize,
    labels_to_position_map: HashMap<Label, usize>,
}

impl EmissionData {
    #[inline(always)]
    pub(crate) fn new(emitted_bytes: usize, labels_to_position_map: HashMap<Label, usize>) -> Self {
        Self {
            emitted_bytes,
            labels_to_position_map,
        }
    }

    /// Releases the internal data of the [`EmissionData`] into a free struct.
    #[inline(always)]
    pub const fn deconstruct(self) -> DeconstructedEmissionData {
        let emitted_bytes = self.emitted_bytes;
        let labels_to_position_map = unsafe { std::ptr::read(&self.labels_to_position_map) };
        forget(self);
        DeconstructedEmissionData {
            emitted_bytes,
            labels_to_position_map,
        }
    }

    /// Returns a map of public labels to their positions in the emitted code,
    /// relative to the beginning of the code, not to the passed stream.
    #[inline(always)]
    pub const fn labels_to_position_map(&self) -> &HashMap<Label, usize> {
        &self.labels_to_position_map
    }

    /// Returns the number of bytes emitted.
    #[inline(always)]
    pub const fn emitted_bytes(&self) -> usize {
        self.emitted_bytes
    }
}
