/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2015-2023 Dariusz Depta
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * Apache license, Version 2.0
 *
 * Copyright (c) 2015-2023 Dariusz Depta
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
