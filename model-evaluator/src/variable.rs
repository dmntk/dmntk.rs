/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2015-2023 Dariusz Depta, Engos Software
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
 * Copyright (c) 2015-2023 Dariusz Depta, Engos Software
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

use crate::item_definition::ItemDefinitionEvaluator;
use crate::item_definition_type::ItemDefinitionTypeEvaluator;
use crate::model_definitions::{DefInformationItem, DefKey};
use dmntk_feel::values::Value;
use dmntk_feel::{value_null, FeelType, Name};

/// Type of closure that evaluates values from variable definition.
pub type VariableEvaluatorFn = Box<dyn Fn(&Value, &ItemDefinitionEvaluator) -> (Name, Value) + Send + Sync>;

/// Variable properties.
#[derive(Clone)]
pub struct Variable {
  /// Variable's namespace.
  namespace: String,
  /// Variable's name.
  name: Name,
  /// Variable's type reference.
  pub type_ref: String,
  /// Variables FEEL type (evaluated).
  feel_type: FeelType,
}

impl From<&DefInformationItem> for Variable {
  /// Converts [Variable] from [DefInformationItem].
  fn from(value: &DefInformationItem) -> Self {
    Self {
      namespace: value.namespace().to_string(),
      name: value.name().clone(),
      type_ref: value.type_ref().clone(),
      feel_type: FeelType::Any,
    }
  }
}

impl Variable {
  /// Returns variable's namespace.
  pub fn namespace(&self) -> &str {
    &self.namespace
  }

  /// Returns variable's name.
  pub fn name(&self) -> &Name {
    &self.name
  }

  /// Resolves the FEEL type of the variable.
  pub fn resolve_feel_type(&self, item_definition_type_evaluator: &ItemDefinitionTypeEvaluator) -> FeelType {
    item_definition_type_evaluator
      .information_item_type(&self.namespace, &self.type_ref)
      .unwrap_or(FeelType::Any)
  }

  /// Updates the FEEL type of the variable.
  pub fn update_feel_type(&mut self, item_definition_type_evaluator: &ItemDefinitionTypeEvaluator) {
    self.feel_type = item_definition_type_evaluator
      .information_item_type(&self.namespace, &self.type_ref)
      .unwrap_or(FeelType::Any);
  }

  /// Returns variable's FEEL type.
  pub fn feel_type(&self) -> &FeelType {
    &self.feel_type
  }

  ///
  pub fn build_evaluator(&self) -> VariableEvaluatorFn {
    let variable_namespace = self.namespace.clone();
    let variable_name = self.name.clone();
    let variable_type_ref = self.type_ref.clone();
    match variable_type_ref.as_str() {
      "Any" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
        if let Value::Context(ctx) = value {
          if let Some(v) = ctx.get_entry(&variable_name) {
            return (variable_name.clone(), v.clone());
          }
        }
        (variable_name.clone(), value_null!())
      }),
      "Null" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
        if let Value::Context(ctx) = value {
          if let Some(v) = ctx.get_entry(&variable_name) {
            return if let Value::Null(_) = v {
              (variable_name.clone(), v.clone())
            } else {
              (variable_name.clone(), v.coerced(&FeelType::Null))
            };
          }
        }
        (variable_name.clone(), value_null!())
      }),
      "string" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
        if let Value::Context(ctx) = value {
          if let Some(v) = ctx.get_entry(&variable_name) {
            return if let Value::String(_) = v {
              (variable_name.clone(), v.clone())
            } else {
              (variable_name.clone(), v.coerced(&FeelType::String))
            };
          }
        }
        (variable_name.clone(), value_null!())
      }),
      "number" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
        if let Value::Context(ctx) = value {
          if let Some(v) = ctx.get_entry(&variable_name) {
            return if let Value::Number(_) = v {
              (variable_name.clone(), v.clone())
            } else {
              (variable_name.clone(), v.coerced(&FeelType::Number))
            };
          }
        }
        (variable_name.clone(), value_null!())
      }),
      "boolean" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
        if let Value::Context(ctx) = value {
          if let Some(v) = ctx.get_entry(&variable_name) {
            return if let Value::Boolean(_) = v {
              (variable_name.clone(), v.clone())
            } else {
              (variable_name.clone(), v.coerced(&FeelType::Boolean))
            };
          }
        }
        (variable_name.clone(), value_null!())
      }),
      "date" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
        if let Value::Context(ctx) = value {
          if let Some(v) = ctx.get_entry(&variable_name) {
            return if let Value::Date(_) = v {
              (variable_name.clone(), v.clone())
            } else {
              (variable_name.clone(), v.coerced(&FeelType::Date))
            };
          }
        }
        (variable_name.clone(), value_null!())
      }),
      "time" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
        if let Value::Context(ctx) = value {
          if let Some(v) = ctx.get_entry(&variable_name) {
            return if let Value::Time(_) = v {
              (variable_name.clone(), v.clone())
            } else {
              (variable_name.clone(), v.coerced(&FeelType::Time))
            };
          }
        }
        (variable_name.clone(), value_null!())
      }),
      "dateTime" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
        if let Value::Context(ctx) = value {
          if let Some(v) = ctx.get_entry(&variable_name) {
            return if let Value::DateTime(_) = v {
              (variable_name.clone(), v.clone())
            } else {
              (variable_name.clone(), v.coerced(&FeelType::DateTime))
            };
          }
        }
        (variable_name.clone(), value_null!())
      }),
      "dayTimeDuration" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
        if let Value::Context(ctx) = value {
          if let Some(v) = ctx.get_entry(&variable_name) {
            return if let Value::DaysAndTimeDuration(_) = v {
              (variable_name.clone(), v.clone())
            } else {
              (variable_name.clone(), v.coerced(&FeelType::DaysAndTimeDuration))
            };
          }
        }
        (variable_name.clone(), value_null!())
      }),
      "yearMonthDuration" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
        if let Value::Context(ctx) = value {
          if let Some(v) = ctx.get_entry(&variable_name) {
            return if let Value::YearsAndMonthsDuration(_) = v {
              (variable_name.clone(), v.clone())
            } else {
              (variable_name.clone(), v.coerced(&FeelType::YearsAndMonthsDuration))
            };
          }
        }
        (variable_name.clone(), value_null!())
      }),
      _ => Box::new(move |value: &Value, item_definition_evaluator: &ItemDefinitionEvaluator| {
        if let Value::Context(ctx) = value {
          if let Some(entry_value) = ctx.get_entry(&variable_name) {
            let evaluated_value = item_definition_evaluator
              .eval(&DefKey::new(&variable_namespace, &variable_type_ref), entry_value)
              .unwrap_or_else(|| value_null!("input data evaluator: item definition evaluator '{}' not found", variable_type_ref));
            (variable_name.clone(), evaluated_value)
          } else {
            (variable_name.clone(), value_null!("no name {} in context {}", variable_name, ctx))
          }
        } else {
          (variable_name.clone(), value_null!("expected context, actual value is {}", value))
        }
      }),
    }
  }
}
