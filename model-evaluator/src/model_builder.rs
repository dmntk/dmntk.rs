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

use crate::business_knowledge_model::BusinessKnowledgeModelEvaluator;
use crate::decision::DecisionEvaluator;
use crate::decision_service::DecisionServiceEvaluator;
use crate::input_data::InputDataEvaluator;
use crate::input_data_context::InputDataContextEvaluator;
use crate::item_definition::ItemDefinitionEvaluator;
use crate::item_definition_context::ItemDefinitionContextEvaluator;
use crate::item_definition_type::{InformationItemTypes, ItemDefinitionTypeEvaluator};
use crate::model_definitions::{DefDefinitions, DefKey, Invocables};
use dmntk_common::Result;
use dmntk_feel::Name;
use dmntk_model::Definitions;
use std::cell::RefCell;

pub struct EvaluatorBuilders {
    pub input_data_evaluator: InputDataEvaluator,
    pub item_definition_evaluator: ItemDefinitionEvaluator,
    pub business_knowledge_model_evaluator: BusinessKnowledgeModelEvaluator,
    pub decision_evaluator: DecisionEvaluator,
    pub decision_service_evaluator: DecisionServiceEvaluator,
    pub invocables: Invocables,
    pub information_item_types: InformationItemTypes,
}

/// Model builder.
#[derive(Default)]
pub struct ModelBuilder {
    /// Model definitions.
    model_definitions: DefDefinitions,
    /// Input data evaluator builder.
    input_data_evaluator: InputDataEvaluator,
    /// Input data context evaluator builder.
    input_data_context_evaluator: InputDataContextEvaluator,
    /// Item definition evaluator builder.
    item_definition_evaluator: ItemDefinitionEvaluator,
    /// Item definition context evaluator builder.
    item_definition_context_evaluator: ItemDefinitionContextEvaluator,
    /// Item definition type evaluator builder.
    item_definition_type_evaluator: ItemDefinitionTypeEvaluator,
    /// Business knowledge model evaluator builder.
    business_knowledge_model_evaluator: BusinessKnowledgeModelEvaluator,
    /// Decision evaluator builder.
    decision_evaluator: DecisionEvaluator,
    /// Decision service evaluator builder.
    decision_service_evaluator: DecisionServiceEvaluator,
    /// Map of invocables indexed by invocable name.
    invocables: RefCell<Invocables>,
}

impl ModelBuilder {
    /// Adds definitions from specified model.
    pub fn add_model(&mut self, definitions: &Definitions) {
        self.model_definitions.add_model(definitions);
    }

    /// Builds a model based on model definitions.
    pub fn build(&mut self) -> Result<()> {
        self.input_data_evaluator = InputDataEvaluator::new(&self.model_definitions);
        self.input_data_context_evaluator = InputDataContextEvaluator::new(&self.model_definitions);
        self.item_definition_evaluator = ItemDefinitionEvaluator::new(&self.model_definitions)?;
        self.item_definition_context_evaluator =
            ItemDefinitionContextEvaluator::new(&self.model_definitions)?;
        self.item_definition_type_evaluator =
            ItemDefinitionTypeEvaluator::new(&self.model_definitions)?;
        self.business_knowledge_model_evaluator =
            BusinessKnowledgeModelEvaluator::new(&self.model_definitions, self)?;
        self.decision_evaluator = DecisionEvaluator::new(&self.model_definitions, self)?;
        self.decision_service_evaluator =
            DecisionServiceEvaluator::new(&self.model_definitions, self)?;
        Ok(())
    }

    ///
    pub fn input_data_evaluator(&self) -> &InputDataEvaluator {
        &self.input_data_evaluator
    }

    ///
    pub fn input_data_context_evaluator(&self) -> &InputDataContextEvaluator {
        &self.input_data_context_evaluator
    }

    ///
    pub fn item_definition_context_evaluator(&self) -> &ItemDefinitionContextEvaluator {
        &self.item_definition_context_evaluator
    }

    ///
    pub fn item_definition_evaluator(&self) -> &ItemDefinitionEvaluator {
        &self.item_definition_evaluator
    }

    ///
    pub fn item_definition_type_evaluator(&self) -> &ItemDefinitionTypeEvaluator {
        &self.item_definition_type_evaluator
    }

    ///
    pub fn decision_evaluator(&self) -> &DecisionEvaluator {
        &self.decision_evaluator
    }

    ///
    pub fn add_decision_invocable(&self, namespace: String, name: String, def_key: DefKey) {
        self.invocables
            .borrow_mut()
            .add_decision(namespace, name, def_key);
    }

    ///
    pub fn add_bkm_invocable(
        &self,
        namespace: String,
        name: String,
        def_key: DefKey,
        output_variable_name: Name,
    ) {
        self.invocables
            .borrow_mut()
            .add_bkm(namespace, name, def_key, output_variable_name);
    }

    ///
    pub fn add_decision_service_invocable(&self, namespace: String, name: String, def_key: DefKey) {
        self.invocables
            .borrow_mut()
            .add_decision_service(namespace, name, def_key);
    }
}

impl From<ModelBuilder> for EvaluatorBuilders {
    fn from(value: ModelBuilder) -> Self {
        Self {
            input_data_evaluator: value.input_data_evaluator,
            item_definition_evaluator: value.item_definition_evaluator,
            business_knowledge_model_evaluator: value.business_knowledge_model_evaluator,
            decision_evaluator: value.decision_evaluator,
            decision_service_evaluator: value.decision_service_evaluator,
            invocables: value.invocables.into_inner(),
            information_item_types: value
                .item_definition_type_evaluator
                .information_item_types(),
        }
    }
}
