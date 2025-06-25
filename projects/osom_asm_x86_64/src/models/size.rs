#![allow(non_upper_case_globals)]

use osom_encoders_x86_64::models as enc_models;

/// Represents the size of a general purpose register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[must_use]
pub struct Size {
    value: enc_models::Size,
}

impl Size {
    pub const Bit8: Self = Self::new(enc_models::Size::Bit8);
    pub const Bit16: Self = Self::new(enc_models::Size::Bit16);
    pub const Bit32: Self = Self::new(enc_models::Size::Bit32);
    pub const Bit64: Self = Self::new(enc_models::Size::Bit64);

    #[inline(always)]
    pub(crate) const fn new(size: enc_models::Size) -> Self {
        Self { value: size }
    }

    #[inline(always)]
    #[must_use]
    pub const fn equals(self, other: Self) -> bool {
        self.value.equals(other.value)
    }

    #[inline(always)]
    pub(crate) fn as_enc_size(&self) -> enc_models::Size {
        self.value
    }
}

impl From<enc_models::Size> for Size {
    #[inline(always)]
    fn from(size: enc_models::Size) -> Self {
        Self::new(size)
    }
}

impl From<Size> for enc_models::Size {
    #[inline(always)]
    fn from(size: Size) -> Self {
        size.value
    }
}
