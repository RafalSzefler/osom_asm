use crate::models::Label;

/// Errors returned during instruction emission.
#[derive(Debug)]
#[repr(u8)]
#[must_use]
pub enum EmitError {
    /// The operands in the instruction have incorrect size.
    OperandSizeMismatch,

    /// Tried to emit the same lable twice.
    LabelAlreadyDefined(Label),
}

/// Errors returned during final assembly.
#[derive(Debug)]
#[repr(u8)]
#[must_use]
pub enum AssembleError {
    /// Errors forwarded from the underlying stream.
    IoError(std::io::Error),

    /// The code contains references to labels that were not set.
    LabelNotSet(Label),
}

impl From<std::io::Error> for AssembleError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}
