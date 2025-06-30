use super::Size;

/// Thin wrapper around a 64-bit signed immediate value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct Immediate64 {
    value: i64,
}

impl Immediate64 {
    #[inline(always)]
    pub const fn new(value: i64) -> Self {
        Self { value }
    }

    #[inline]
    pub const fn real_size(self) -> Size {
        // While this implementation looks weird, it actually results
        // in Rust compiler optimizing this into cmov instructions.
        // Instead of branches, if classical fast return is used.
        let value = self.value;
        let mut result = Size::Bit64;
        if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
            result = Size::Bit32;
        }
        if value >= i16::MIN as i64 && value <= i16::MAX as i64 {
            result = Size::Bit16;
        }
        if value >= i8::MIN as i64 && value <= i8::MAX as i64 {
            result = Size::Bit8;
        }
        result
    }

    #[inline(always)]
    #[must_use]
    pub const fn value(self) -> i64 {
        self.value
    }
}

impl From<i64> for Immediate64 {
    #[inline(always)]
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<Immediate64> for i64 {
    #[inline(always)]
    fn from(immediate: Immediate64) -> Self {
        immediate.value()
    }
}
