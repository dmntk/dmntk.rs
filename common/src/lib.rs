//! # Common definitions used by components of **DMNTK** project

#[macro_use]
extern crate dmntk_macros;

mod ascii_ctrl;
mod ascii_tree;
mod errors;
mod href;
mod idents;
mod jsonify;
mod namespace;
mod uri;

pub use ascii_ctrl::*;
pub use ascii_tree::*;
pub use errors::{DmntkError, Result, ToErrorMessage};
pub use href::HRef;
pub use idents::gen_id;
pub use jsonify::Jsonify;
pub use namespace::to_rdnn;
pub use uri::{to_uri, Uri};
