use super::Size;

/// Thin wrapper around a 32-bit signed immediate value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct Immediate {
    value: i32,
}

impl Immediate {
    pub const ZERO: Self = Self::new(0);

    #[inline(always)]
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    #[inline]
    pub const fn real_size(self) -> Size {
        // While this implementation looks weird, it actually results
        // in Rust compiler optimizing this into cmov instructions.
        // Instead of branches, if classical fast return is used.
        let value = self.value;
        let mut result = Size::Bit32;
        if value >= i16::MIN as i32 && value <= i16::MAX as i32 {
            result = Size::Bit16;
        }
        if value >= i8::MIN as i32 && value <= i8::MAX as i32 {
            result = Size::Bit8;
        }
        result
    }

    #[inline(always)]
    #[must_use]
    pub const fn value(self) -> i32 {
        self.value
    }
}

impl From<i32> for Immediate {
    #[inline(always)]
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl From<Immediate> for i32 {
    #[inline(always)]
    fn from(immediate: Immediate) -> Self {
        immediate.value()
    }
}
