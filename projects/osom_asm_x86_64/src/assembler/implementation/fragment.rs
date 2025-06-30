#![allow(non_camel_case_types)]
use crate::models::{Condition, Label};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct FragmentOrderId {
    value: u32,
}

impl FragmentOrderId {
    pub fn from_index(offset: u32) -> Self {
        Self { value: offset }
    }

    pub fn index(self) -> u32 {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RelaxationVariant {
    Short,
    Long,
}

#[derive(Debug)]
pub enum Fragment {
    Bytes {
        data_length: u32,
        capacity: u32,
    },
    Relaxable_Jump {
        variant: RelaxationVariant,
        label: Label,
    },
    Relaxable_CondJump {
        variant: RelaxationVariant,
        condition: Condition,
        label: Label,
    },
}

pub mod const_sizes {
    use osom_encoders_x86_64::encoders as enc;
    use osom_encoders_x86_64::models as enc_models;

    pub const SHORT_JUMP: u32 = const {
        enc::jmp::encode_jmp_imm8(enc_models::Immediate8::from_i8(0))
            .as_slice()
            .len() as u32
    };
    pub const LONG_JUMP: u32 = const {
        enc::jmp::encode_jmp_imm32(enc_models::Immediate32::from_i32(0))
            .as_slice()
            .len() as u32
    };
    pub const SHORT_COND_JUMP: u32 = const {
        enc::jcc::encode_jcc_A_imm8(enc_models::Immediate8::from_i8(0))
            .as_slice()
            .len() as u32
    };
    pub const LONG_COND_JUMP: u32 = const {
        enc::jcc::encode_jcc_A_imm32(enc_models::Immediate32::from_i32(0))
            .as_slice()
            .len() as u32
    };

    const _CHECK: () = const {
        // The reason we do this checks is because perhaps in the future
        // we will overwrite the existing buffer with real instructions.
        // So we want to be sure we have space for that. This is not
        // implemented at the moment.
        assert!(SHORT_JUMP as usize <= size_of::<super::Fragment>());
        assert!(LONG_JUMP as usize <= size_of::<super::Fragment>());
        assert!(SHORT_COND_JUMP as usize <= size_of::<super::Fragment>());
        assert!(LONG_COND_JUMP as usize <= size_of::<super::Fragment>());
    };
}

impl Fragment {
    #[allow(clippy::cast_ptr_alignment)]
    pub unsafe fn next(&self) -> *mut Fragment {
        let offset = match self {
            Fragment::Bytes { capacity, .. } => *capacity as usize,
            _ => size_of::<Fragment>(),
        };
        let raw_ptr = std::ptr::from_ref::<Fragment>(self).cast_mut().cast::<u8>();
        unsafe { raw_ptr.add(offset) }.cast::<Fragment>()
    }

    #[inline(always)]
    pub fn slice_of_header(&self) -> &[u8] {
        let ptr = std::ptr::from_ref::<Fragment>(self).cast::<u8>();
        let len = size_of::<Fragment>();
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    pub fn data_length(&self) -> u32 {
        match self {
            Fragment::Bytes { data_length, .. } => *data_length,
            Fragment::Relaxable_Jump { variant, .. } => match variant {
                RelaxationVariant::Short => const_sizes::SHORT_JUMP,
                RelaxationVariant::Long => const_sizes::LONG_JUMP,
            },
            Fragment::Relaxable_CondJump { variant, .. } => match variant {
                RelaxationVariant::Short => const_sizes::SHORT_COND_JUMP,
                RelaxationVariant::Long => const_sizes::LONG_COND_JUMP,
            },
        }
    }
}
