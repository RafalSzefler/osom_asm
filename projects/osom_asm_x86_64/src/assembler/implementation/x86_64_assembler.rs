use std::collections::HashMap;

use osom_asm_common::InlineVec;
use osom_encoders_x86_64::models::EncodedX86_64Instruction;

use crate::assembler::implementation::fragment::RelaxationVariant;
use crate::assembler::EmitError;
use crate::models::Label;

use super::fragment::{Fragment, FragmentOrderId};
use super::macros::{fragment_at_index, fragment_at_index_mut};

#[derive(Debug, Clone)]
pub(super) struct LabelOffset {
    pub fragment_id: FragmentOrderId,
    pub in_fragment_offset: u32,
}

#[derive(Clone)]
pub struct X86_64Assembler {
    pub(super) label_offsets: HashMap<Label, LabelOffset>,
    pub(super) patchable_binaries_addresses: HashMap<Label, InlineVec<LabelOffset, 4>>,
    pub(super) public_labels: Vec<Label>,
    pub(super) fragments: Vec<u8>,
    pub(super) last_fragment_offset: u32,
    pub(super) fragments_count: u32,
    pub(super) with_relaxation: bool,
}

const FRAGMENT_SIZE: u32 = size_of::<Fragment>() as u32;
const FRAGMENT_ALIGNMENT: u32 = align_of::<Fragment>() as u32;

impl X86_64Assembler {
    #[inline(always)]
    pub fn new(with_relaxation: bool) -> Self {
        let mut fragments = Vec::<u8>::with_capacity(1 << 12);
        let initial_fragment = Fragment::Bytes { data_length: 0, capacity: FRAGMENT_SIZE };
        fragments.extend_from_slice(initial_fragment.slice_of_header());

        Self {
            label_offsets: HashMap::with_capacity(16),
            patchable_binaries_addresses: HashMap::with_capacity(16),
            public_labels: Vec::with_capacity(4),
            fragments: fragments,
            last_fragment_offset: 0,
            fragments_count: 1,
            with_relaxation,
        }
    }

    pub(super) fn _write_bytes_internal(&mut self, bytes: &[u8]) -> Result<(), EmitError> {
        if bytes.is_empty() {
            return Ok(());
        }
        let bytes_len = bytes.len() as u32;
        let current_fragment = fragment_at_index_mut!(self, self.last_fragment_offset);
        match current_fragment {
            Fragment::Bytes { data_length, capacity } => {
                *data_length += bytes_len;
                *capacity = (((*data_length + FRAGMENT_SIZE) / FRAGMENT_ALIGNMENT) + 1) * FRAGMENT_ALIGNMENT;
            },
            _ => {
                let new_fragment = Fragment::Bytes { data_length: 0, capacity: FRAGMENT_SIZE };
                self._push_new_fragment(new_fragment);

                // Reload fragment.
                let Fragment::Bytes { data_length, capacity } = fragment_at_index_mut!(self, self.last_fragment_offset) else {
                    panic!("New fragment is not a bytes fragment.");
                };
                *data_length += bytes_len;
                *capacity = (((*data_length + FRAGMENT_SIZE) / FRAGMENT_ALIGNMENT) + 1) * FRAGMENT_ALIGNMENT;
            }
        }
        self.fragments.extend_from_slice(bytes);
        Ok(())
    }

    pub(super) fn current_label_offset(&self) -> LabelOffset {
        let current_fragment = fragment_at_index!(self, self.last_fragment_offset);
        let offset = match current_fragment {
            Fragment::Bytes { data_length, .. } => *data_length,
            _ => 0
        };

        let fragment_order_id = FragmentOrderId::from_index(self.last_fragment_offset);

        LabelOffset {
            fragment_id: fragment_order_id,
            in_fragment_offset: offset,
        }
    }

    pub(super) fn _push_new_fragment(&mut self, fragment: Fragment) {
        let current_fragment = fragment_at_index!(self, self.last_fragment_offset);
        let padding = match current_fragment {
            Fragment::Bytes { data_length, capacity} => *capacity - *data_length - FRAGMENT_SIZE,
            _ => 0
        };
        debug_assert!(padding <= FRAGMENT_ALIGNMENT, "Padding is too large, expected at most {} bytes, got {}", FRAGMENT_ALIGNMENT, padding);
        if padding > 0 {
            let buffer = [0; FRAGMENT_ALIGNMENT as usize];
            let slice = &buffer[..padding as usize];
            self.fragments.extend_from_slice(slice);
        }
        self.last_fragment_offset = self.fragments.len() as u32;
        self.fragments.extend_from_slice(fragment.slice_of_header());
        self.fragments_count += 1;
    }

    #[inline(always)]
    pub(super) const fn _relaxation_variant(&self) -> RelaxationVariant {
        if self.with_relaxation { RelaxationVariant::Short } else { RelaxationVariant::Long }
    }

    pub(super) fn _insert_label(&mut self, label: Label) -> Result<(), EmitError> {
        if self.label_offsets.contains_key(&label) {
            return Err(EmitError::LabelAlreadyDefined(label));
        }
        let label_offset = self.current_label_offset();
        self.label_offsets.insert(label, label_offset);
        Ok(())
    }
}

impl Default for X86_64Assembler {
    fn default() -> Self {
        Self::new(true)
    }
}
