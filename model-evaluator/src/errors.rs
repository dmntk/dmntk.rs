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

use dmntk_common::{DmntkError, ToErrorMessage};
use dmntk_feel::FeelType;

/// Errors related to model evaluation.
#[derive(ToErrorMessage)]
struct ModelEvaluatorError(String);

pub fn err_business_knowledge_model_with_reference_not_found(namespace: &str, id: &str) -> DmntkError {
  ModelEvaluatorError(format!("no business knowledge model with reference: '{namespace}#{id}'")).into()
}

pub fn err_empty_literal_expression() -> DmntkError {
  ModelEvaluatorError("empty literal expression".into()).into()
}

pub fn err_empty_encapsulated_logic() -> DmntkError {
  ModelEvaluatorError("empty encapsulated logic in business knowledge model".into()).into()
}

pub fn err_invalid_item_definition_type(s: &str) -> DmntkError {
  ModelEvaluatorError(format!("invalid item definition type for '{s}'")).into()
}

pub fn err_unsupported_feel_type(feel_type: FeelType, s: &str) -> DmntkError {
  ModelEvaluatorError(format!("unsupported FEEL type: {feel_type} in {s}")).into()
}

pub fn err_empty_feel_type() -> DmntkError {
  ModelEvaluatorError("empty FEEL type".into()).into()
}

pub fn err_empty_function_body() -> DmntkError {
  ModelEvaluatorError("empty function definition body".into()).into()
}
