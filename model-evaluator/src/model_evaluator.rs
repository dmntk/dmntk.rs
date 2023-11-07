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

//! # Decision model evaluator

use crate::business_knowledge_model::BusinessKnowledgeModelEvaluator;
use crate::decision::DecisionEvaluator;
use crate::decision_service::DecisionServiceEvaluator;
use crate::input_data::InputDataEvaluator;
use crate::item_definition::ItemDefinitionEvaluator;
use crate::model_builder::{EvaluatorBuilders, ModelBuilder};
use crate::model_definitions::{DefKey, InvocableType, Invocables};
use dmntk_common_1::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::{value_null, Name};
use dmntk_model::Definitions;
use std::sync::Arc;

/// Model evaluator.
pub struct ModelEvaluator {
    /// Input data evaluator.
    input_data_evaluator: InputDataEvaluator,
    /// Item definition evaluator.
    item_definition_evaluator: ItemDefinitionEvaluator,
    /// Business knowledge model evaluator.
    business_knowledge_model_evaluator: BusinessKnowledgeModelEvaluator,
    /// Decision evaluator.
    decision_evaluator: DecisionEvaluator,
    /// Decision service evaluator.
    decision_service_evaluator: DecisionServiceEvaluator,
    /// Map of invocables indexed by invocable name.
    invocables: Invocables,
    /// Map of global information item types defined in this model evaluator.
    global_context: FeelContext,
}

impl From<ModelBuilder> for ModelEvaluator {
    /// Creates [ModelEvaluator] from provided [ModelBuilder].
    fn from(model_builder: ModelBuilder) -> Self {
        let builders: EvaluatorBuilders = model_builder.into();
        let mut global_context = FeelContext::default();
        for (def_key, feel_type) in builders.information_item_types {
            global_context.set_entry(&Name::from(def_key.id()), Value::FeelType(feel_type))
        }
        Self {
            input_data_evaluator: builders.input_data_evaluator,
            item_definition_evaluator: builders.item_definition_evaluator,
            business_knowledge_model_evaluator: builders.business_knowledge_model_evaluator,
            decision_evaluator: builders.decision_evaluator,
            decision_service_evaluator: builders.decision_service_evaluator,
            invocables: builders.invocables,
            global_context,
        }
    }
}

impl ModelEvaluator {
    /// Creates an instance of [ModelEvaluator] from parsed [Definitions].
    pub fn new(definitions: &[Definitions]) -> Result<Arc<Self>> {
        let mut model_builder = ModelBuilder::default();
        definitions
            .iter()
            .for_each(|definitions| model_builder.add_model(definitions));
        model_builder.build()?;
        let model_evaluator: Arc<ModelEvaluator> = Arc::new(model_builder.into());
        model_evaluator
            .decision_service_evaluator
            .build_function_definitions(&Arc::clone(&model_evaluator));
        Ok(model_evaluator)
    }

    /// Returns a reference to input data evaluator.
    pub fn input_data_evaluator(&self) -> &InputDataEvaluator {
        &self.input_data_evaluator
    }

    /// Returns a reference to item definition evaluator.
    pub fn item_definition_evaluator(&self) -> &ItemDefinitionEvaluator {
        &self.item_definition_evaluator
    }

    /// Returns a reference to business knowledge model evaluator.
    pub fn business_knowledge_model_evaluator(&self) -> &BusinessKnowledgeModelEvaluator {
        &self.business_knowledge_model_evaluator
    }

    /// Returns a reference to decision evaluator.
    pub fn decision_evaluator(&self) -> &DecisionEvaluator {
        &self.decision_evaluator
    }

    /// Returns a reference to decision service evaluator.
    pub fn decision_service_evaluator(&self) -> &DecisionServiceEvaluator {
        &self.decision_service_evaluator
    }

    /// Returns a reference to invocables in model evaluator.
    pub fn invocables(&self) -> &Invocables {
        &self.invocables
    }

    /// Evaluates an invocable.
    pub fn evaluate_invocable(
        &self,
        namespace: &str,
        invocable_name: &str,
        input_data: &FeelContext,
    ) -> Value {
        let Some(invocable) = self.invocables.by_name(namespace, invocable_name) else {
      return value_null!("invocable '{}' not found in namespace '{}'", invocable_name, namespace);
    };
        match invocable {
            InvocableType::Decision(def_key) => {
                // evaluate a decision
                self.evaluate_decision(def_key, input_data)
            }
            InvocableType::BusinessKnowledgeModel(def_key, output_variable_name) => {
                // evaluate a business knowledge model
                self.evaluate_bkm(def_key, input_data, output_variable_name)
            }
            InvocableType::DecisionService(def_key) => {
                // evaluate a decision service
                self.evaluate_decision_service(def_key, input_data)
            }
        }
    }

    /// Evaluates a decision.
    fn evaluate_decision(&self, def_key: &DefKey, input_data: &FeelContext) -> Value {
        let mut evaluated_ctx = FeelContext::default();
        if let Some(output_variable_name) = self.decision_evaluator.evaluate(
            def_key,
            &self.global_context,
            input_data,
            self,
            &mut evaluated_ctx,
        ) {
            if let Some(output_value) = evaluated_ctx.get_entry(&output_variable_name) {
                output_value.clone()
            } else {
                value_null!()
            }
        } else {
            value_null!()
        }
    }

    /// Evaluates a business knowledge model.
    fn evaluate_bkm(
        &self,
        def_key: &DefKey,
        input_data: &FeelContext,
        output_variable_name: &Name,
    ) -> Value {
        let mut evaluated_ctx = FeelContext::default();
        self.business_knowledge_model_evaluator.evaluate(
            def_key,
            &self.global_context,
            input_data,
            self,
            &mut evaluated_ctx,
        );
        if let Some(Value::FunctionDefinition(
            parameters,
            body,
            _external,
            _,
            closure_ctx,
            result_type,
        )) = evaluated_ctx.get_entry(output_variable_name)
        {
            //TODO Handle external functions.
            let mut parameters_ctx = FeelContext::default();
            parameters_ctx.zip(closure_ctx);
            for (name, _) in parameters {
                if let Some(value) = input_data.get_entry(name) {
                    parameters_ctx.set_entry(name, value.to_owned());
                }
            }
            parameters_ctx.zip(&evaluated_ctx);
            let result = body.evaluate(&parameters_ctx.into());
            result.coerced(result_type)
        } else {
            value_null!()
        }
    }

    /// Evaluates a decision service.
    fn evaluate_decision_service(&self, def_key: &DefKey, input_data: &FeelContext) -> Value {
        let mut evaluated_ctx = FeelContext::default();
        if let Some(output_variable_name) = self.decision_service_evaluator.evaluate(
            def_key,
            &self.global_context,
            input_data,
            self,
            &mut evaluated_ctx,
        ) {
            if let Some(output_value) = evaluated_ctx.get_entry(&output_variable_name) {
                output_value.clone()
            } else {
                value_null!()
            }
        } else {
            value_null!()
        }
    }
}
