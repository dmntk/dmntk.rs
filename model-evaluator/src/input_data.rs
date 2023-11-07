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

//! # Builder for input data evaluators

use crate::item_definition::ItemDefinitionEvaluator;
use crate::model_definitions::{DefDefinitions, DefKey};
use crate::variable::{Variable, VariableEvaluatorFn};
use dmntk_feel::values::Value;
use dmntk_feel::Name;
use std::collections::HashMap;
use std::sync::Arc;

///
pub type InputDataEvaluatorEntry = (Variable, VariableEvaluatorFn);

/// Input data evaluator.
#[derive(Default)]
pub struct InputDataEvaluator {
  evaluators: Arc<HashMap<DefKey, InputDataEvaluatorEntry>>,
}

impl InputDataEvaluator {
  /// Builds a new input data evaluator.
  pub fn new(definitions: &DefDefinitions) -> Self {
    let mut evaluators = HashMap::new();
    for input_data in definitions.input_data() {
      let input_data_namespace = input_data.namespace();
      let input_data_id = input_data.id();
      let variable: Variable = input_data.variable().into();
      let evaluator = variable.build_evaluator();
      let def_key = DefKey::new(input_data_namespace, input_data_id);
      evaluators.insert(def_key, (variable, evaluator));
    }
    Self { evaluators: Arc::new(evaluators) }
  }

  /// Evaluates input data.
  pub fn evaluate(&self, def_key: &DefKey, value: &Value, item_definition_evaluator: &ItemDefinitionEvaluator) -> Option<(Name, Value)> {
    self.evaluators.get(def_key).map(|evaluator| evaluator.1(value, item_definition_evaluator))
  }

  /// Returns the variable for input data definition.
  pub fn get_variable(&self, def_key: &DefKey) -> Option<&Variable> {
    self.evaluators.get(def_key).map(|entry| &entry.0)
  }
}

#[cfg(test)]
mod tests {
  use crate::input_data::InputDataEvaluator;
  use crate::item_definition::ItemDefinitionEvaluator;
  use crate::model_definitions::{DefDefinitions, DefKey};
  use dmntk_examples::input_data::*;
  use dmntk_feel::values::Value;
  use dmntk_feel::{value_number, FeelNumber, Name};

  const NAMESPACE: &str = "https://dmntk.io";

  /// Utility function for building input data evaluator from definitions,
  /// and item definition evaluator from definitions.
  fn build_evaluators(xml: &str) -> (InputDataEvaluator, ItemDefinitionEvaluator) {
    let definitions = dmntk_model::parse(xml).unwrap();
    let mut def_definitions = DefDefinitions::default();
    def_definitions.add_model(&definitions);
    (InputDataEvaluator::new(&def_definitions), ItemDefinitionEvaluator::new(&def_definitions).unwrap())
  }

  #[test]
  fn _0001_1() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0001);
    let context_str = r#"{Full Name: "John"}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Full", "Name"]), Value::String("John".to_string()))),
      input_data_evaluator.evaluate(
        &DefKey::new(NAMESPACE, "_cba86e4d-e91c-46a2-9176-e9adf88e15db"),
        &Value::Context(context),
        &item_definitions_evaluator
      )
    );
    let context_str = r#"{Full Name: "Phillip"}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Full", "Name"]), Value::String("Phillip".to_string()))),
      input_data_evaluator.evaluate(
        &DefKey::new(NAMESPACE, "_cba86e4d-e91c-46a2-9176-e9adf88e15db"),
        &Value::Context(context),
        &item_definitions_evaluator
      )
    );
  }

  #[test]
  fn _0001_2() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0001);
    let context_str = r#"{Full Name: 50.0}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Full", "Name"]), Value::Null(Some("after coercion".to_string())))),
      input_data_evaluator.evaluate(
        &DefKey::new(NAMESPACE, "_cba86e4d-e91c-46a2-9176-e9adf88e15db"),
        &Value::Context(context),
        &item_definitions_evaluator
      )
    );
  }

  #[test]
  fn _0002_1() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0002);
    let context_str = r#"{Monthly Salary: 12000.00}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Monthly", "Salary"]), value_number!(12000))),
      input_data_evaluator.evaluate(
        &DefKey::new(NAMESPACE, "_b7a53bad-7a5b-4033-841d-5db6b25834ad"),
        &Value::Context(context),
        &item_definitions_evaluator
      )
    );
    let context_str = r#"{Monthly Salary: 8135.35}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Monthly", "Salary"]), value_number!(813535, 2))),
      input_data_evaluator.evaluate(
        &DefKey::new(NAMESPACE, "_b7a53bad-7a5b-4033-841d-5db6b25834ad"),
        &Value::Context(context),
        &item_definitions_evaluator
      )
    );
  }

  #[test]
  fn _0002_2() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0002);
    let context_str = r#"{Monthly Salary: "12000.00"}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Monthly", "Salary"]), Value::Null(Some("after coercion".to_string())))),
      input_data_evaluator.evaluate(
        &DefKey::new(NAMESPACE, "_b7a53bad-7a5b-4033-841d-5db6b25834ad"),
        &Value::Context(context),
        &item_definitions_evaluator
      )
    );
  }

  #[test]
  fn _0003_1() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0003);
    let context_str = r#"{Is Affordable: true}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Is", "Affordable"]), Value::Boolean(true))),
      input_data_evaluator.evaluate(
        &DefKey::new(NAMESPACE, "_b7a53bad-7a5b-4033-841d-5db6b25834ad"),
        &Value::Context(context),
        &item_definitions_evaluator
      )
    );
    let context_str = r#"{Is Affordable: false}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Is", "Affordable"]), Value::Boolean(false))),
      input_data_evaluator.evaluate(
        &DefKey::new(NAMESPACE, "_b7a53bad-7a5b-4033-841d-5db6b25834ad"),
        &Value::Context(context),
        &item_definitions_evaluator
      )
    );
  }

  #[test]
  fn _0003_2() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0003);
    let context_str = r#"{Is Affordable: "no"}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Is", "Affordable"]), Value::Null(Some("after coercion".to_string())))),
      input_data_evaluator.evaluate(
        &DefKey::new(NAMESPACE, "_b7a53bad-7a5b-4033-841d-5db6b25834ad"),
        &Value::Context(context),
        &item_definitions_evaluator
      )
    );
  }

  #[test]
  fn _0103_1() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0103);
    let context_str = r#"{Employment Status: "EMPLOYED"}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let name = Name::new(&["Employment", "Status"]);
    assert_eq!(
      Some((name, Value::String("EMPLOYED".to_string()))),
      input_data_evaluator.evaluate(
        &DefKey::new(NAMESPACE, "_acfd4e1d-da0a-4842-aa35-ea50dd36fb01"),
        &Value::Context(context),
        &item_definitions_evaluator
      )
    );
  }
}
