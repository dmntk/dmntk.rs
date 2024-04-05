//! # Error definitions for TCK handler

use dmntk_common::{DmntkError, ToErrorMessage};

/// Server errors for TCK handler.
#[derive(ToErrorMessage)]
struct TckServerError(String);

pub fn err_missing_attribute(name: &str) -> DmntkError {
  TckServerError(format!("missing attribute: {name}")).into()
}
