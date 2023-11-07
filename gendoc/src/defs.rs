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

//! `CSS` common definitions.

//--------------------------------------------------------------------------------------------------
// COMMON DEFINITIONS
//--------------------------------------------------------------------------------------------------

pub const DIAGRAM_SHARED_STYLE_PREFIX: &str = "diagram-shared-style-";

//--------------------------------------------------------------------------------------------------
// CSS FILES
//--------------------------------------------------------------------------------------------------

/// Content of the `CSS` stylesheet for DMN model.
pub const DMN_MODEL_CSS: &str = include_str!("templates/dmn-model.css");

/// Content of the `CSS` stylesheet for decision tables.
pub const DECISION_TABLE_CSS: &str = include_str!("templates/decision-table.css");

//--------------------------------------------------------------------------------------------------
// CSS DEFINITIONS
//--------------------------------------------------------------------------------------------------

/// Class name for the container for diagram.
pub const CLASS_DIAGRAM_CONTAINER: Option<&str> = Some("diagram-container");

/// Class name for diagram title.
pub const CLASS_DIAGRAM_TITLE: Option<&str> = Some("diagram-title");

/// Class name for the container for model element details.
pub const CLASS_MODEL_ELEMENT_CONTAINER: Option<&str> = Some("model-element-container");

/// Class name for the name of the model element.
pub const CLASS_MODEL_ELEMENT_NAME: Option<&str> = Some("model-element-name");

/// Class name for the type of the model element.
pub const CLASS_MODEL_ELEMENT_TYPE: Option<&str> = Some("model-element-type");

/// Class name for the element displaying `description`.
pub const CLASS_DESCRIPTION: Option<&str> = Some("description");

/// Class name for the element containing a `description`.
pub const CLASS_DESCRIPTION_CONTAINER: Option<&str> = Some("description-container");

/// Class name for the container for expression instance details.
pub const CLASS_EXPRESSION_INSTANCE_CONTAINER: Option<&str> = Some("expression-instance-container");

///
pub const CLASS_DECISION_TABLE: Option<&str> = Some("decision-table");

///
pub const CLASS_INFORMATION_ITEM_NAME: Option<&str> = Some("information-item-name");

///
pub const CLASS_GRID_CONTAINER: Option<&str> = Some("grid-container");

///
pub const CLASS_GRID_BODY: Option<&str> = Some("grid-body");

///
pub const CLASS_HORIZONTAL_DOUBLE_LINE: Option<&str> = Some("horizontal-double-line");

///
pub const CLASS_VERTICAL_DOUBLE_LINE: Option<&str> = Some("vertical-double-line");

///
pub const CLASS_HIT_POLICY: Option<&str> = Some("hit-policy");

///
pub const CLASS_INPUT_EXPRESSION: Option<&str> = Some("input-expression");

///
pub const CLASS_INPUT_ALLOWED_VALUES: Option<&str> = Some("input-allowed-values");

///
pub const CLASS_INPUT_ENTRY: Option<&str> = Some("input-entry");

///
pub const CLASS_OUTPUT_LABEL: Option<&str> = Some("output-label");

///
pub const CLASS_OUTPUT_COMPONENT: Option<&str> = Some("output-component");

///
pub const CLASS_OUTPUT_ALLOWED_VALUES: Option<&str> = Some("output-allowed-values");

///
pub const CLASS_OUTPUT_ENTRY: Option<&str> = Some("output-entry");

///
pub const CLASS_RULE_NUMBER: Option<&str> = Some("rule-number");

///
pub const CLASS_ANNOTATION_LABEL: Option<&str> = Some("annotation-label");

///
pub const CLASS_ANNOTATION_ALLOWED_VALUES: Option<&str> = Some("annotation-allowed-values");

///
pub const CLASS_ANNOTATION_ENTRY: Option<&str> = Some("annotation-entry");

//--------------------------------------------------------------------------------------------------
// HTML DEFINITIONS
//--------------------------------------------------------------------------------------------------

/// Text of the heading displayed before model diagrams section.
pub const HEADING_MODEL_DIAGRAMS: &str = "Model diagrams";

/// Text of the heading displayed before model elements section.
pub const HEADING_MODEL_ELEMENTS: &str = "Model elements";

/// Text of the heading displayed before the output variable properties.
pub const HEADING_OUTPUT_DATA: &str = "Output data";

/// Text of the heading displayed before the input variable properties.
pub const HEADING_INPUT_DATA: &str = "Input data";
