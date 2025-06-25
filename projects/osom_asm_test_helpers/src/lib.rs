#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::module_name_repetitions,
    clippy::len_without_is_empty,
    clippy::should_implement_trait,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

pub mod macros;

#[doc(hidden)]
pub mod _hidden;
