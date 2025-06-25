use crate::models::Label;

#[derive(Debug)]
#[repr(u8)]
#[must_use]
pub enum EmitError {
    OutOfMemory,
    OperandSizeMismatch,
    LabelAlreadyDefined(Label),
}

#[derive(Debug)]
#[repr(u8)]
#[must_use]
pub enum AssembleError {
    IoError(std::io::Error),
    LabelNotSet(Label),
}

impl From<std::io::Error> for AssembleError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}
