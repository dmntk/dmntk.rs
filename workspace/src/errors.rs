//! Errors reported by workspace.

use dmntk_common::{DmntkError, ToErrorMessage};

/// Errors reported by workspace.
#[derive(ToErrorMessage)]
struct WorkspaceError(String);

pub fn err_invocable_not_found(invocable_path: &str) -> DmntkError {
  WorkspaceError(format!("invocable not found: '{invocable_path}'")).into()
}
