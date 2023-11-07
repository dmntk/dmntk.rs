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

use dmntk_common_1::{DmntkError, ToErrorMessage};

/// `FEEL` expressions evaluator errors.
#[derive(ToErrorMessage)]
struct FeelEvaluatorError(String);

pub fn err_not_a_context() -> DmntkError {
    FeelEvaluatorError("expected FEEL context as an input".to_string()).into()
}

pub fn err_expected_positional_or_named_parameter() -> DmntkError {
    FeelEvaluatorError("expected positional or named parameter".to_string()).into()
}

pub fn err_expected_ast_node_parameter_name(s: &str) -> DmntkError {
    FeelEvaluatorError(format!(
        "expected AstNode::ParameterName, actual node is {s}"
    ))
    .into()
}

pub fn err_expected_ast_node(expected: &str, actual: &str) -> DmntkError {
    FeelEvaluatorError(format!(
        "expected AST node {expected}, actual AST node is {actual}"
    ))
    .into()
}

pub fn err_unexpected_ast_node(s: &str) -> DmntkError {
    FeelEvaluatorError(format!("unexpected AST node in evaluator builder {s}")).into()
}
