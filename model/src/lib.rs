#[macro_use]
extern crate dmntk_macros;

mod errors;
mod model;
mod parser;
mod validator;
mod xml_utils;

#[cfg(test)]
mod tests;

pub use model::*;
pub use parser::parse;
