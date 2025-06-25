use osom_encoders_x86_64::models as enc_models;

use super::{GPRKind, Size};

/// Represents an error that occurs when creating a new general purpose register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NewGPRError {
    /// Error when creating a new `GPR` from a [`GPRKind::Bit8High`] and `index` outside of the `4..=7` range.
    InvalidBit8HighIndex,

    /// Error when creating a new `GPR` from a `kind` and `index` outside of the `0..=15` range.
    IndexOutOfRange,
}

/// Represents an `x86_64` general purpose register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct GPR {
    value: enc_models::GPR,
}

impl GPR {
    pub const RAX: Self = unsafe { Self::new_unchecked(enc_models::GPR::RAX) };
    pub const RCX: Self = unsafe { Self::new_unchecked(enc_models::GPR::RCX) };
    pub const RDX: Self = unsafe { Self::new_unchecked(enc_models::GPR::RDX) };
    pub const RBX: Self = unsafe { Self::new_unchecked(enc_models::GPR::RBX) };
    pub const RSP: Self = unsafe { Self::new_unchecked(enc_models::GPR::RSP) };
    pub const RBP: Self = unsafe { Self::new_unchecked(enc_models::GPR::RBP) };
    pub const RSI: Self = unsafe { Self::new_unchecked(enc_models::GPR::RSI) };
    pub const RDI: Self = unsafe { Self::new_unchecked(enc_models::GPR::RDI) };
    pub const R8: Self = unsafe { Self::new_unchecked(enc_models::GPR::R8) };
    pub const R9: Self = unsafe { Self::new_unchecked(enc_models::GPR::R9) };
    pub const R10: Self = unsafe { Self::new_unchecked(enc_models::GPR::R10) };
    pub const R11: Self = unsafe { Self::new_unchecked(enc_models::GPR::R11) };
    pub const R12: Self = unsafe { Self::new_unchecked(enc_models::GPR::R12) };
    pub const R13: Self = unsafe { Self::new_unchecked(enc_models::GPR::R13) };
    pub const R14: Self = unsafe { Self::new_unchecked(enc_models::GPR::R14) };
    pub const R15: Self = unsafe { Self::new_unchecked(enc_models::GPR::R15) };

    pub const EAX: Self = unsafe { Self::new_unchecked(enc_models::GPR::EAX) };
    pub const ECX: Self = unsafe { Self::new_unchecked(enc_models::GPR::ECX) };
    pub const EDX: Self = unsafe { Self::new_unchecked(enc_models::GPR::EDX) };
    pub const EBX: Self = unsafe { Self::new_unchecked(enc_models::GPR::EBX) };
    pub const ESP: Self = unsafe { Self::new_unchecked(enc_models::GPR::ESP) };
    pub const EBP: Self = unsafe { Self::new_unchecked(enc_models::GPR::EBP) };
    pub const ESI: Self = unsafe { Self::new_unchecked(enc_models::GPR::ESI) };
    pub const EDI: Self = unsafe { Self::new_unchecked(enc_models::GPR::EDI) };
    pub const R8D: Self = unsafe { Self::new_unchecked(enc_models::GPR::R8D) };
    pub const R9D: Self = unsafe { Self::new_unchecked(enc_models::GPR::R9D) };
    pub const R10D: Self = unsafe { Self::new_unchecked(enc_models::GPR::R10D) };
    pub const R11D: Self = unsafe { Self::new_unchecked(enc_models::GPR::R11D) };
    pub const R12D: Self = unsafe { Self::new_unchecked(enc_models::GPR::R12D) };
    pub const R13D: Self = unsafe { Self::new_unchecked(enc_models::GPR::R13D) };
    pub const R14D: Self = unsafe { Self::new_unchecked(enc_models::GPR::R14D) };
    pub const R15D: Self = unsafe { Self::new_unchecked(enc_models::GPR::R15D) };

    pub const AX: Self = unsafe { Self::new_unchecked(enc_models::GPR::AX) };
    pub const CX: Self = unsafe { Self::new_unchecked(enc_models::GPR::CX) };
    pub const DX: Self = unsafe { Self::new_unchecked(enc_models::GPR::DX) };
    pub const BX: Self = unsafe { Self::new_unchecked(enc_models::GPR::BX) };
    pub const SP: Self = unsafe { Self::new_unchecked(enc_models::GPR::SP) };
    pub const BP: Self = unsafe { Self::new_unchecked(enc_models::GPR::BP) };
    pub const SI: Self = unsafe { Self::new_unchecked(enc_models::GPR::SI) };
    pub const DI: Self = unsafe { Self::new_unchecked(enc_models::GPR::DI) };
    pub const R8W: Self = unsafe { Self::new_unchecked(enc_models::GPR::R8W) };
    pub const R9W: Self = unsafe { Self::new_unchecked(enc_models::GPR::R9W) };
    pub const R10W: Self = unsafe { Self::new_unchecked(enc_models::GPR::R10W) };
    pub const R11W: Self = unsafe { Self::new_unchecked(enc_models::GPR::R11W) };
    pub const R12W: Self = unsafe { Self::new_unchecked(enc_models::GPR::R12W) };
    pub const R13W: Self = unsafe { Self::new_unchecked(enc_models::GPR::R13W) };
    pub const R14W: Self = unsafe { Self::new_unchecked(enc_models::GPR::R14W) };
    pub const R15W: Self = unsafe { Self::new_unchecked(enc_models::GPR::R15W) };

    pub const AL: Self = unsafe { Self::new_unchecked(enc_models::GPR::AL) };
    pub const CL: Self = unsafe { Self::new_unchecked(enc_models::GPR::CL) };
    pub const DL: Self = unsafe { Self::new_unchecked(enc_models::GPR::DL) };
    pub const BL: Self = unsafe { Self::new_unchecked(enc_models::GPR::BL) };
    pub const SPL: Self = unsafe { Self::new_unchecked(enc_models::GPR::SPL) };
    pub const BPL: Self = unsafe { Self::new_unchecked(enc_models::GPR::BPL) };
    pub const SIL: Self = unsafe { Self::new_unchecked(enc_models::GPR::SIL) };
    pub const DIL: Self = unsafe { Self::new_unchecked(enc_models::GPR::DIL) };
    pub const R8B: Self = unsafe { Self::new_unchecked(enc_models::GPR::R8B) };
    pub const R9B: Self = unsafe { Self::new_unchecked(enc_models::GPR::R9B) };
    pub const R10B: Self = unsafe { Self::new_unchecked(enc_models::GPR::R10B) };
    pub const R11B: Self = unsafe { Self::new_unchecked(enc_models::GPR::R11B) };
    pub const R12B: Self = unsafe { Self::new_unchecked(enc_models::GPR::R12B) };
    pub const R13B: Self = unsafe { Self::new_unchecked(enc_models::GPR::R13B) };
    pub const R14B: Self = unsafe { Self::new_unchecked(enc_models::GPR::R14B) };
    pub const R15B: Self = unsafe { Self::new_unchecked(enc_models::GPR::R15B) };

    pub const AH: Self = unsafe { Self::new_unchecked(enc_models::GPR::AH) };
    pub const CH: Self = unsafe { Self::new_unchecked(enc_models::GPR::CH) };
    pub const DH: Self = unsafe { Self::new_unchecked(enc_models::GPR::DH) };
    pub const BH: Self = unsafe { Self::new_unchecked(enc_models::GPR::BH) };

    pub(crate) const unsafe fn new_unchecked(gpr: enc_models::GPR) -> Self {
        Self { value: gpr }
    }

    #[inline]
    pub fn new(kind: GPRKind, index: u8) -> Result<Self, NewGPRError> {
        if kind.equals(GPRKind::Bit8High) && !(4..=7).contains(&index) {
            return Err(NewGPRError::InvalidBit8HighIndex);
        }

        if index > 15 {
            return Err(NewGPRError::IndexOutOfRange);
        }

        unsafe {
            let enc_gpr = enc_models::GPR::new_unchecked(kind.into(), index);
            Ok(Self::new_unchecked(enc_gpr))
        }
    }

    #[inline(always)]
    pub fn kind(self) -> GPRKind {
        GPRKind::from(self.value.kind())
    }

    #[inline(always)]
    pub fn size(self) -> Size {
        Size::from(self.value.size())
    }

    #[inline(always)]
    pub(crate) fn as_enc_gpr(self) -> enc_models::GPR {
        self.value
    }

    #[inline(always)]
    pub(crate) fn as_enc_mem(self) -> enc_models::GPROrMemory {
        enc_models::GPROrMemory::GPR { gpr: self.value }
    }
}

impl From<enc_models::GPR> for GPR {
    #[inline(always)]
    fn from(gpr: enc_models::GPR) -> Self {
        unsafe { Self::new_unchecked(gpr) }
    }
}

impl From<GPR> for enc_models::GPR {
    #[inline(always)]
    fn from(gpr: GPR) -> Self {
        gpr.value
    }
}
