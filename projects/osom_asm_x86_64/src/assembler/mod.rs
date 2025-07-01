//! The main module that contains and implements the assembler.
mod emission_data;
pub use emission_data::*;

mod errors;
pub use errors::*;

mod traits;

mod implementation;
pub use implementation::*;
