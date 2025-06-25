use osom_encoders_x86_64::models as enc_models;


use super::{GPR, Immediate, Label, Scale, Size};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
#[repr(u8)]
pub(crate) enum MemoryImpl {
    Based {
        base: GPR,
        offset: Immediate,
    },
    Scaled {
        index: GPR,
        scale: Scale,
        offset: Immediate,
    },
    BasedScaled {
        base: GPR,
        index: GPR,
        scale: Scale,
        offset: Immediate,
    },
    Label {
        label: Label,
    },
}

/// Represents a general `x86_64` memory operand.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
#[repr(transparent)]
pub struct Memory {
    value: MemoryImpl,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
#[must_use]
pub enum NewMemoryError {
    GPRNotBit64,
    RSPNotAllowedAsIndex,
}

impl Memory {
    #[inline]
    pub fn based(base: GPR, offset: Immediate) -> Result<Self, NewMemoryError> {
        if base.size() != Size::Bit64 {
            return Err(NewMemoryError::GPRNotBit64);
        }

        Ok(Self {
            value: MemoryImpl::Based { base, offset },
        })
    }

    #[inline]
    pub fn scaled(index: GPR, scale: Scale, offset: Immediate) -> Result<Self, NewMemoryError> {
        if index.size() != Size::Bit64 {
            return Err(NewMemoryError::GPRNotBit64);
        }

        if index == GPR::RSP {
            return Err(NewMemoryError::RSPNotAllowedAsIndex);
        }

        Ok(Self {
            value: MemoryImpl::Scaled { index, scale, offset },
        })
    }

    #[inline]
    pub fn based_scaled(base: GPR, index: GPR, scale: Scale, offset: Immediate) -> Result<Self, NewMemoryError> {
        if base.size() != Size::Bit64 {
            return Err(NewMemoryError::GPRNotBit64);
        }

        if index.size() != Size::Bit64 {
            return Err(NewMemoryError::GPRNotBit64);
        }

        if index == GPR::RSP {
            return Err(NewMemoryError::RSPNotAllowedAsIndex);
        }

        Ok(Self {
            value: MemoryImpl::BasedScaled {
                base,
                index,
                scale,
                offset,
            },
        })
    }

    /// This will get translated to RIP-relative address.
    #[inline(always)]
    pub const fn label(label: Label) -> Self {
        Self {
            value: MemoryImpl::Label { label },
        }
    }

    #[inline(always)]
    pub(crate) fn as_enc_mem(&self) -> enc_models::Memory {
        const fn imm_to_offset(offset: &Immediate) -> enc_models::Offset {
            let val = offset.value();
            if val == 0 {
                return enc_models::Offset::None;
            }

            match offset.real_size() {
                Size::Bit8 => enc_models::Offset::Bit8(enc_models::Immediate8::from_i8(val as i8)),
                Size::Bit16 | Size::Bit32 => enc_models::Offset::Bit32(enc_models::Immediate32::from_i32(val as i32)),
                _ => panic!("Invalid offset size"),
            }
        }

        match &self.value {
            MemoryImpl::Based { base, offset } => {
                enc_models::Memory::Based { base: base.as_enc_gpr(), offset: imm_to_offset(offset) }
            },
            MemoryImpl::Scaled { index, scale, offset } => {
                enc_models::Memory::Scaled { index: index.as_enc_gpr(), scale: scale.as_enc_scale(), offset: imm_to_offset(offset) }
            },
            MemoryImpl::BasedScaled { base, index, scale, offset } => {
                enc_models::Memory::BasedScaled { base: base.as_enc_gpr(), index: index.as_enc_gpr(), scale: scale.as_enc_scale(), offset: imm_to_offset(offset) }
            },
            MemoryImpl::Label { label: _ } => {
                // We ignore the label. We emit the code with the offset 0,
                // and the correct label offset will be patched later.
                enc_models::Memory::RelativeToRIP { offset: enc_models::Offset::None }
            },
        }
    }
}
