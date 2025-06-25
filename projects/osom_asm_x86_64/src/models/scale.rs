#![allow(non_upper_case_globals)]

use osom_encoders_x86_64::models as enc_models;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
#[repr(transparent)]
pub struct Scale {
    value: enc_models::Scale,
}

impl Scale {
    pub const Scale1: Self = Self::new(enc_models::Scale::Scale1);
    pub const Scale2: Self = Self::new(enc_models::Scale::Scale2);
    pub const Scale4: Self = Self::new(enc_models::Scale::Scale4);
    pub const Scale8: Self = Self::new(enc_models::Scale::Scale8);

    #[inline(always)]
    pub(crate) const fn new(value: enc_models::Scale) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub(crate) fn as_enc_scale(&self) -> enc_models::Scale {
        self.value
    }
}
