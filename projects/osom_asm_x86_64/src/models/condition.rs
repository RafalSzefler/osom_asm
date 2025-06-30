/// Represents `X86_64` conditions usable with certain
/// instructions, e.g. conditional jumps and/or cmov.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
#[repr(u8)]
pub enum Condition {
    Equal = 1,

    NotEqual,

    /// In unsigned sense.
    Above,

    /// In unsigned sense.
    AboveOrEqual,

    /// In unsigned sense.
    Below,

    /// In unsigned sense.
    BelowOrEqual,

    /// In signed sense.
    Greater,

    /// In signed sense.
    GreaterOrEqual,

    /// In signed sense.
    Less,

    /// In signed sense.
    LessOrEqual,

    Overflow,

    NotOverflow,

    Parity,

    NotParity,

    ParityOdd,

    ParityEven,

    Sign,

    NotSign,

    Carry,

    NotCarry,
}
