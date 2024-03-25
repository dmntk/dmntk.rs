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

use crate::builders::BuildContext;
use crate::errors::err_not_a_context;
use dmntk_common::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::{Evaluator, FeelScope};
use dmntk_feel_parser::AstNode;

/// Evaluates a [Value] from given [AstNode].
pub fn evaluate(scope: &FeelScope, node: &AstNode) -> Result<Value> {
  let evaluator = crate::builders::build_evaluator(&BuildContext::default(), node)?;
  Ok(evaluator(scope))
}

/// Prepares an evaluator for given [AstNode].
pub fn prepare(bx: &BuildContext, node: &AstNode) -> Result<Evaluator> {
  crate::builders::build_evaluator(bx, node)
}

/// Evaluates the sum of specified values.
pub fn evaluate_sum(values: Vec<Value>) -> Value {
  crate::bifs::core::sum(&values)
}

/// Evaluates the minimum value from specified values.
pub fn evaluate_min(values: Vec<Value>) -> Value {
  crate::bifs::core::min(&values)
}

/// Evaluates the maximum value from specified values.
pub fn evaluate_max(values: Vec<Value>) -> Value {
  crate::bifs::core::max(&values)
}

/// Compares two values and returns `true` when the two `FEEL` values are equal.
pub fn evaluate_equals(left: &Value, right: &Value) -> bool {
  crate::builders::eval_ternary_equality(left, right).unwrap_or(false)
}

/// Evaluates a context from text containing `FEEL` expression.
pub fn evaluate_context(scope: &FeelScope, input: &str) -> Result<FeelContext> {
  let node = &dmntk_feel_parser::parse_context(scope, input, false)?;
  evaluate_context_node(scope, node)
}

/// Evaluates a context from AST node.
pub fn evaluate_context_node(scope: &FeelScope, node: &AstNode) -> Result<FeelContext> {
  let evaluator = crate::builders::build_evaluator(&BuildContext::default(), node)?;
  if let Value::Context(context) = evaluator(scope) {
    Ok(context)
  } else {
    Err(err_not_a_context())
  }
}
