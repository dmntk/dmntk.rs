//! # Container for decision model evaluators

use crate::builder::WorkspaceBuilder;
use crate::errors::*;
use dmntk_common::{ColorPalette, Result};
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_model_evaluator::ModelEvaluator;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

/// Container for decision model evaluators.
pub struct Workspaces {
  /// Map: invocable path -> (workspace name, namespace, invocable name)
  pub(crate) invocables: HashMap<String, (String, String, String)>,
  /// Map: workspace name -> model evaluator
  pub(crate) evaluators: HashMap<String, Arc<ModelEvaluator>>,
}

impl Workspaces {
  /// Creates a new [Workspaces] and loads decision models from specified directory.
  pub fn new(dir: &Path, colors: ColorPalette, verbose: bool) -> Self {
    let mut builder = WorkspaceBuilder::new(colors, verbose);
    builder.load_decision_models(dir);
    Self {
      invocables: builder.invocables,
      evaluators: builder.evaluators,
    }
  }

  /// Evaluates invocable identified by invocable path.
  pub fn evaluate(&self, invocable_path: &str, input_data: &FeelContext) -> Result<Value> {
    if let Some((workspace, namespace, invocable_name)) = self.invocables.get(invocable_path) {
      if let Some(evaluator) = self.evaluators.get(workspace) {
        return Ok(evaluator.evaluate_invocable(namespace, invocable_name, input_data));
      }
    }
    Err(err_invocable_not_found(invocable_path))
  }
}
