#![allow(non_upper_case_globals)]

use super::Size;
use osom_encoders_x86_64::models as enc_models;

/// Represents the kind of a general purpose register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct GPRKind {
    value: enc_models::GPRKind,
}

impl GPRKind {
    /// Represents AH, BH, CH and DH registers.
    pub const Bit8High: Self = Self::new(enc_models::GPRKind::Bit8High);

    /// Represents AL, CL, DL, BL, SPL, BPL, SIL, DIL, R8B, R9B, R10B, R11B, R12B, R13B, R14B and R15B registers.
    pub const Bit8: Self = Self::new(enc_models::GPRKind::Bit8);

    /// Represents AX, CX, DX, BX, SP, BP, SI, DI, R8W, R9W, R10W, R11W, R12W, R13W, R14W and R15W registers.
    pub const Bit16: Self = Self::new(enc_models::GPRKind::Bit16);

    /// Represents EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI, R8D, R9D, R10D, R11D, R12D, R13D, R14D and R15D registers.
    pub const Bit32: Self = Self::new(enc_models::GPRKind::Bit32);

    /// Represents RAX, RCX, RDX, RBX, RSP, RBP, RSI, RDI, R8, R9, R10, R11, R12, R13, R14 and R15 registers.
    pub const Bit64: Self = Self::new(enc_models::GPRKind::Bit64);

    #[inline(always)]
    pub(crate) const fn new(gpr: enc_models::GPRKind) -> Self {
        Self { value: gpr }
    }

    #[inline(always)]
    pub const fn size(self) -> Size {
        Size::new(self.value.size())
    }

    #[inline(always)]
    #[must_use]
    pub const fn equals(self, other: Self) -> bool {
        self.value.equals(other.value)
    }

    #[inline(always)]
    pub(crate) fn as_enc_gpr_kind(self) -> enc_models::GPRKind {
        self.value
    }
}

impl From<enc_models::GPRKind> for GPRKind {
    #[inline(always)]
    fn from(gpr: enc_models::GPRKind) -> Self {
        Self { value: gpr }
    }
}

impl From<GPRKind> for enc_models::GPRKind {
    #[inline(always)]
    fn from(gpr: GPRKind) -> Self {
        gpr.value
    }
}
