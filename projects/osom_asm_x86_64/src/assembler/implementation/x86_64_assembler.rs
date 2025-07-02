#![allow(clippy::cast_ptr_alignment)]

use std::collections::HashMap;

use osom_tools_runtime::InlineVec;

use crate::assembler::EmitError;
use crate::assembler::implementation::fragment::RelaxationVariant;
use crate::models::Label;

use super::fragment::{Fragment, FragmentOrderId};
use super::macros::{fragment_at_index, fragment_at_index_mut};

#[derive(Debug, Clone)]
#[must_use]
pub(super) struct FragmentRelativePosition {
    pub fragment_id: FragmentOrderId,
    pub in_fragment_offset: i32,
}

#[derive(Debug, Clone)]
#[must_use]
pub(super) struct PatchableImm32Instruction {
    pub instruction_position: FragmentRelativePosition,
    pub instruction_length: u8,
    pub imm32_offset: u8,
}

/// The main `X86_64` assembler.
///
/// This assembler can be created in two modes: with or without relaxation.
/// Relaxation is an optmization technique that allows the assembler to emit
/// shorter encoding for certain instructions. This algorithm however is not
/// free, and in fact of quadratic complexity. Thus the option is switchable.
///
/// Note that relaxation is enabled by default.
#[derive(Clone)]
#[must_use]
pub struct X86_64Assembler {
    pub(super) label_offsets: HashMap<Label, FragmentRelativePosition>,
    pub(super) patchable_addresses: HashMap<Label, InlineVec<PatchableImm32Instruction, 5>>,
    pub(super) public_labels: Vec<Label>,
    pub(super) fragments: Vec<u8>,
    pub(super) last_fragment_offset: i32,
    pub(super) fragments_count: u32,
    pub(super) with_relaxation: bool,
}

#[allow(clippy::cast_possible_wrap)]
const FRAGMENT_SIZE: i32 = const {
    let size = size_of::<Fragment>();
    assert!(
        size > 0 && size <= i32::MAX as usize,
        "Fragment size is too large, doesn't fit in i32."
    );
    size as i32
};

#[allow(clippy::cast_possible_wrap)]
const FRAGMENT_ALIGNMENT: i32 = const {
    let alignment = align_of::<Fragment>();
    assert!(
        alignment > 0 && alignment <= i32::MAX as usize,
        "Fragment alignment is too large, doesn't fit in i32."
    );
    alignment as i32
};

const MAX_BYTE_ARRAY_LENGTH: usize = (i32::MAX - 2048) as usize;

impl X86_64Assembler {
    /// Creates a new `X86_64` assembler.
    ///
    /// # Arguments
    ///
    /// * `with_relaxation` - whether to enable relaxation optimization or not.
    #[inline(always)]
    pub(super) fn new(with_relaxation: bool, predefined_labels: HashMap<Label, FragmentRelativePosition>) -> Self {
        let mut fragments = Vec::<u8>::with_capacity(1 << 12);
        let initial_fragment = Fragment::Bytes {
            data_length: 0,
            capacity: FRAGMENT_SIZE,
        };
        fragments.extend_from_slice(initial_fragment.slice_of_header());

        Self {
            label_offsets: predefined_labels,
            patchable_addresses: HashMap::with_capacity(16),
            public_labels: Vec::with_capacity(4),
            fragments: fragments,
            last_fragment_offset: 0,
            fragments_count: 1,
            with_relaxation,
        }
    }

    pub(super) fn _write_bytes_internal(&mut self, bytes: &[u8]) {
        if bytes.is_empty() {
            return;
        }

        assert!(
            bytes.len() <= MAX_BYTE_ARRAY_LENGTH,
            "Bytes length is too large, expected at most {} bytes, got {}.",
            MAX_BYTE_ARRAY_LENGTH,
            bytes.len()
        );

        #[allow(clippy::cast_possible_wrap)]
        let bytes_len = bytes.len() as i32;

        let current_fragment = fragment_at_index_mut!(self, self.last_fragment_offset);
        if let Fragment::Bytes { data_length, capacity } = current_fragment {
            *data_length += bytes_len;
            *capacity = (((*data_length + FRAGMENT_SIZE) / FRAGMENT_ALIGNMENT) + 1) * FRAGMENT_ALIGNMENT;
        } else {
            let new_fragment = Fragment::Bytes {
                data_length: 0,
                capacity: FRAGMENT_SIZE,
            };
            self._push_new_fragment(new_fragment);
            let Fragment::Bytes { data_length, capacity } = fragment_at_index_mut!(self, self.last_fragment_offset)
            else {
                panic!("New fragment is not a bytes fragment.");
            };
            *data_length += bytes_len;
            *capacity = (((*data_length + FRAGMENT_SIZE) / FRAGMENT_ALIGNMENT) + 1) * FRAGMENT_ALIGNMENT;
        }

        self.fragments.extend_from_slice(bytes);
    }

    pub(super) fn _current_position(&self) -> FragmentRelativePosition {
        let current_fragment = fragment_at_index!(self, self.last_fragment_offset);
        let offset = match current_fragment {
            Fragment::Bytes { data_length, .. } => *data_length,
            _ => 0,
        };

        let fragment_order_id = FragmentOrderId::from_index(self.last_fragment_offset);

        FragmentRelativePosition {
            fragment_id: fragment_order_id,
            in_fragment_offset: offset,
        }
    }

    #[allow(clippy::needless_pass_by_value, clippy::checked_conversions)]
    pub(super) fn _push_new_fragment(&mut self, fragment: Fragment) {
        let current_fragment = fragment_at_index!(self, self.last_fragment_offset);
        let padding = match current_fragment {
            Fragment::Bytes { data_length, capacity } => *capacity - *data_length - FRAGMENT_SIZE,
            _ => 0,
        };
        assert!(
            padding <= FRAGMENT_ALIGNMENT,
            "Padding is too large, expected at most {FRAGMENT_ALIGNMENT} bytes, got {padding}"
        );

        #[allow(clippy::cast_sign_loss)]
        if padding > 0 {
            let buffer = [0; FRAGMENT_ALIGNMENT as usize];
            let slice = &buffer[..padding as usize];

            {
                let new_size = self.fragments.len() + padding as usize;
                assert!(
                    new_size <= i32::MAX as usize,
                    "Fragments length is too large, got {new_size} which doesn't fit in i32."
                );
            }

            self.fragments.extend_from_slice(slice);
        }

        let fragment_as_slice = fragment.slice_of_header();

        {
            let new_size = self.fragments.len() + fragment_as_slice.len();
            assert!(
                new_size <= i32::MAX as usize,
                "Fragments length is too large, got {new_size} which doesn't fit in i32."
            );
        }

        #[allow(clippy::cast_possible_wrap)]
        {
            self.last_fragment_offset = self.fragments.len() as i32;
        }
        self.fragments.extend_from_slice(fragment_as_slice);
        self.fragments_count += 1;
    }

    #[inline(always)]
    pub(super) const fn _relaxation_variant(&self) -> RelaxationVariant {
        if self.with_relaxation {
            RelaxationVariant::Short
        } else {
            RelaxationVariant::Long
        }
    }

    pub(super) fn _insert_label(&mut self, label: Label) -> Result<(), EmitError> {
        if self.label_offsets.contains_key(&label) {
            return Err(EmitError::LabelAlreadyDefined(label));
        }
        let label_offset = self._current_position();
        self.label_offsets.insert(label, label_offset);
        Ok(())
    }

    #[inline(always)]
    pub(super) fn _push_patchable_instruction(&mut self, label: Label, patch_info: PatchableImm32Instruction) {
        self.patchable_addresses.entry(label).or_default().push(patch_info);
    }
}
