//! The main module that contains and implements the assembler.
#![allow(unused_imports)]

mod emission_data;
pub use emission_data::*;

mod errors;
pub use errors::*;

mod traits;
pub use traits::*;

mod implementation;
pub use implementation::*;
