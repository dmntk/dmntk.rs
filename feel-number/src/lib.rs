//! Implementation of the `FEEL` number type.

#[macro_use]
extern crate dmntk_macros;

mod dfp_number;
mod errors;

pub use dfp_number::FeelNumber;
