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

//! DMN™ documentation generator in `HTML` format.

use crate::defs::*;
use crate::horizontal_decision_table::create_horizontal_decision_table_elements;
use dmntk_model::*;
use domrs::*;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

const PI_2: f64 = std::f64::consts::PI * 2.0;
const AMPLITUDE: f64 = 20.0;

/// Generates `HTML` document for specified DMN™ model definitions.
pub fn dmn_model_to_html(definitions: &Definitions) -> String {
  let mut body = HtmlElement::new("body");
  // add section with model title
  let document_title = definitions.name();
  body.add_child(create_html_heading(HeadingLevel::H1, document_title));
  body.add_child_opt(create_description_in_container(definitions.description()));
  // add model diagrams
  body.add_child(create_html_heading(HeadingLevel::H2, HEADING_MODEL_DIAGRAMS));
  for html_element in create_model_diagrams(definitions) {
    body.add_child(html_element);
  }
  // add model elements
  body.add_child(create_html_heading(HeadingLevel::H2, HEADING_MODEL_ELEMENTS));
  for html_element in create_model_elements(definitions) {
    body.add_child(html_element);
  }
  let diagram_shared_styles = diagram_shared_styles(definitions);
  HtmlDocument::new(document_title, "en", &[DMN_MODEL_CSS, DECISION_TABLE_CSS, &diagram_shared_styles], body).to_string()
}

struct CssClass {
  name: String,
  attributes: BTreeMap<String, String>,
}

impl Display for CssClass {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, ".{} {{", self.name)?;
    for (name, value) in &self.attributes {
      writeln!(f, "  {}: {};", name, value)?
    }
    writeln!(f, "}}")
  }
}

impl CssClass {
  fn new(name: String) -> Self {
    Self {
      name,
      attributes: BTreeMap::new(),
    }
  }

  fn add_attribute(&mut self, name: &str, value: &str) {
    self.attributes.insert(name.to_string(), value.to_string());
  }
}

struct CssClasses {
  classes: Vec<CssClass>,
}

impl CssClasses {
  fn new() -> Self {
    Self { classes: vec![] }
  }

  fn add_class(&mut self, css_class: CssClass) {
    self.classes.push(css_class);
  }
}

impl Display for CssClasses {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    for css_class in &self.classes {
      write!(f, "{}", css_class)?
    }
    Ok(())
  }
}

fn diagram_shared_styles(definitions: &Definitions) -> String {
  let mut css_classes = CssClasses::new();
  if let Some(dmndi) = definitions.dmndi() {
    for style in &dmndi.styles {
      if let Some(style_id) = &style.id {
        let mut css_class = CssClass::new(format!("{}{}", DIAGRAM_SHARED_STYLE_PREFIX, style_id));
        if style.font_bold {
          css_class.add_attribute("font-weight", "500");
        }
        if style.font_underline {
          css_class.add_attribute("text-decoration", "underline");
        }
        if style.font_strike_through {
          css_class.add_attribute("text-decoration", "line-through");
        }
        if let Some(font_size) = style.font_size {
          css_class.add_attribute("font-size", &format!("{}pt", font_size));
        }
        //TODO handle lacking attributes
        css_classes.add_class(css_class);
      }
    }
  }
  format!("{}", css_classes)
}

/// Generates `HTML` document for specified decision table.
pub fn decision_table_to_html(decision_table: &DecisionTable) -> String {
  let mut body = HtmlElement::new("body");
  // add title
  let document_title = if let Some(information_item_name) = &decision_table.information_item_name() {
    information_item_name
  } else if let Some(output_label) = &decision_table.output_label() {
    output_label
  } else {
    "Decision Table"
  };
  body.add_child(create_html_heading(HeadingLevel::H1, document_title));
  // add decision table
  match &decision_table.preferred_orientation() {
    DecisionTableOrientation::RuleAsRow => {
      body.add_child(create_horizontal_decision_table_elements(decision_table));
    }
    DecisionTableOrientation::RuleAsColumn => {
      //
    }
    DecisionTableOrientation::CrossTable => {
      //
    }
  }
  HtmlDocument::new(document_title, "en", &[DMN_MODEL_CSS, DECISION_TABLE_CSS], body).to_string()
}

/// Creates a collection of model diagrams.
fn create_model_diagrams(definitions: &Definitions) -> Vec<HtmlElement> {
  let mut html_elements = vec![]; // collection of sections, each section contains singe diagram
  if let Some(dmndi) = definitions.dmndi() {
    for diagram in &dmndi.diagrams {
      let mut html_svg_content = vec![];
      for diagram_element in &diagram.diagram_elements {
        match diagram_element {
          DmnDiagramElement::DmnShape(shape) => {
            if let Some(dmn_element_ref) = &shape.dmn_element_ref {
              if let Some(decision) = definitions.get_decision(dmn_element_ref.as_str()) {
                html_svg_content.push(create_svg_decision(shape, decision));
              } else if let Some(input_data) = definitions.get_input_data(dmn_element_ref.as_str()) {
                html_svg_content.push(create_svg_input_data(shape, input_data));
              } else if let Some(business_knowledge_model) = definitions.get_business_knowledge_model(dmn_element_ref.as_str()) {
                html_svg_content.push(create_svg_business_knowledge_model(shape, business_knowledge_model));
              } else if let Some(knowledge_source) = definitions.get_knowledge_source(dmn_element_ref.as_str()) {
                html_svg_content.push(create_svg_knowledge_source(shape, knowledge_source));
              }
            }
          }
          DmnDiagramElement::DmnEdge(edge) => {
            if let Some(id) = &edge.dmn_element_ref {
              if let Some(requirement) = definitions.get_requirement(id) {
                match requirement {
                  Requirement::Information(_) => {
                    html_svg_content.push(create_svg_edge_solid_with_black_arrow(&edge.way_points));
                  }
                  Requirement::Knowledge(_) => {
                    html_svg_content.push(create_svg_edge_dashed_with_thin_arrow(&edge.way_points));
                  }
                  Requirement::Authority(_) => {
                    html_svg_content.push(create_svg_edge_dashed_with_end_point(&edge.way_points));
                  }
                }
              }
            }
          }
        }
      }
      html_elements.push(create_svg_tag(&diagram.name, &diagram.size, html_svg_content));
    }
  }
  html_elements
}

/// Creates a collection of model elements.
fn create_model_elements(definitions: &Definitions) -> Vec<HtmlElement> {
  let mut html_elements = vec![];
  for decision_service in &definitions.decision_services() {
    html_elements.push(create_model_element_decision_service(decision_service));
  }
  for decision in &definitions.decisions() {
    html_elements.push(create_model_element_decision(decision));
  }
  for bkm in &definitions.business_knowledge_models() {
    html_elements.push(create_model_element_business_knowledge_model(bkm));
  }
  for knowledge_source in &definitions.knowledge_sources() {
    html_elements.push(create_model_element_knowledge_source(knowledge_source));
  }
  for input_data in &definitions.input_data() {
    html_elements.push(create_model_element_input_data(input_data));
  }
  html_elements
}

/// Creates model element containing decision service details.
fn create_model_element_decision_service(decision_service: &DecisionService) -> HtmlElement {
  let mut element_name = HtmlElement::new_div(CLASS_MODEL_ELEMENT_NAME);
  element_name.set_content(decision_service.name());
  let mut element_type = HtmlElement::new_div(CLASS_MODEL_ELEMENT_TYPE);
  element_type.set_content("(Decision Service)");
  let variable_details = create_variable_details(decision_service.variable(), HEADING_OUTPUT_DATA);
  //
  let mut model_element_container = HtmlElement::new_div(CLASS_MODEL_ELEMENT_CONTAINER);
  model_element_container.add_children(vec![element_name, element_type, variable_details]);
  model_element_container
}

/// Creates model element containing decision details.
fn create_model_element_decision(decision: &Decision) -> HtmlElement {
  // prepare the container for the details
  let mut model_element_container = HtmlElement::new_div(CLASS_MODEL_ELEMENT_CONTAINER);
  // prepare the name of the element
  let mut element_name = HtmlElement::new_div(CLASS_MODEL_ELEMENT_NAME);
  element_name.set_content(decision.name());
  model_element_container.add_child(element_name);
  // prepare the type of the element
  let mut element_type = HtmlElement::new_div(CLASS_MODEL_ELEMENT_TYPE);
  element_type.set_content("(Decision)");
  model_element_container.add_child(element_type);
  // prepare description for the decision when provided
  model_element_container.add_child_opt(create_description_in_container(decision.description()));
  // prepare output variable details
  let variable_details = create_variable_details(decision.variable(), HEADING_OUTPUT_DATA);
  model_element_container.add_child(variable_details);
  // prepare the decision logic details
  model_element_container.add_child_opt(create_model_expression_instance(decision.decision_logic()));
  // return the container with filled up content
  model_element_container
}

/// Creates model element containing business knowledge model details.
fn create_model_element_business_knowledge_model(bkm: &BusinessKnowledgeModel) -> HtmlElement {
  let mut element_name = HtmlElement::new_div(CLASS_MODEL_ELEMENT_NAME);
  element_name.set_content(bkm.name());
  let mut element_type = HtmlElement::new_div(CLASS_MODEL_ELEMENT_TYPE);
  element_type.set_content("(Business Knowledge Model)");
  let variable_details = create_variable_details(bkm.variable(), HEADING_OUTPUT_DATA);
  //
  let mut model_element_container = HtmlElement::new_div(CLASS_MODEL_ELEMENT_CONTAINER);
  model_element_container.add_children(vec![element_name, element_type, variable_details]);
  model_element_container
}

/// Creates model element containing knowledge source details.
fn create_model_element_knowledge_source(knowledge_source: &KnowledgeSource) -> HtmlElement {
  let mut element_name = HtmlElement::new_div(CLASS_MODEL_ELEMENT_NAME);
  element_name.set_content(knowledge_source.name());
  let mut element_type = HtmlElement::new_div(CLASS_MODEL_ELEMENT_TYPE);
  element_type.set_content("(Knowledge Source)");
  let mut model_element_container = HtmlElement::new_div(CLASS_MODEL_ELEMENT_CONTAINER);
  model_element_container.add_children(vec![element_name, element_type]);
  model_element_container
}

/// Creates model element containing input data details.
fn create_model_element_input_data(input_data: &InputData) -> HtmlElement {
  let mut element_name = HtmlElement::new_div(CLASS_MODEL_ELEMENT_NAME);
  element_name.set_content(input_data.name());
  let mut element_type = HtmlElement::new_div(CLASS_MODEL_ELEMENT_TYPE);
  element_type.set_content("(Input Data)");
  let variable_details = create_variable_details(input_data.variable(), HEADING_INPUT_DATA);
  //
  let mut model_element_container = HtmlElement::new_div(CLASS_MODEL_ELEMENT_CONTAINER);
  model_element_container.add_children(vec![element_name, element_type, variable_details]);
  model_element_container
}

/// Creates element containing input/output variable details.
fn create_variable_details(variable: &InformationItem, heading: &str) -> HtmlElement {
  let mut variable_details_heading = HtmlElement::new_div(Some("variable-details-heading"));
  variable_details_heading.set_content(heading);

  let mut variable_details_properties = HtmlElement::new_div(Some("variable-details-properties"));

  if let Some(label) = variable.label() {
    let mut property_label = HtmlElement::new_div(Some("variable-details-property-name"));
    property_label.set_content("Label");
    let mut value_label = HtmlElement::new_div(Some("variable-details-property-value"));
    value_label.set_content(label);
    variable_details_properties.add_children(vec![property_label, value_label]);
  }

  let mut property_name = HtmlElement::new_div(Some("variable-details-property-name"));
  property_name.set_content("Name");
  let mut value_name = HtmlElement::new_div(Some("variable-details-property-value"));
  value_name.set_content(variable.name());
  variable_details_properties.add_children(vec![property_name, value_name]);

  if variable.description().is_some() {
    let mut property_description = HtmlElement::new_div(Some("variable-details-property-name"));
    property_description.set_content("Description");
    let mut value_description = HtmlElement::new_div(Some("variable-details-property-value"));
    value_description.add_child_opt(create_description(variable.description()));
    variable_details_properties.add_children(vec![property_description, value_description]);
  }

  let mut property_type = HtmlElement::new_div(Some("variable-details-property-name"));
  property_type.set_content("Type");
  let mut value_type = HtmlElement::new_div(Some("variable-details-property-value-type"));
  value_type.set_content(variable.type_ref());
  variable_details_properties.add_children(vec![property_type, value_type]);

  let mut variable_details = HtmlElement::new_div(Some("variable-details-container"));
  variable_details.add_children(vec![variable_details_heading, variable_details_properties]);
  variable_details
}

/// Creates element containing expression instance details.
fn create_model_expression_instance(opt_expression_instance: &Option<ExpressionInstance>) -> Option<HtmlElement> {
  if let Some(expression_instance) = opt_expression_instance {
    let mut container = HtmlElement::new_div(CLASS_EXPRESSION_INSTANCE_CONTAINER);
    match expression_instance {
      ExpressionInstance::Context(_) => {
        let mut variable_details_heading = HtmlElement::new_div(Some("variable-details-heading"));
        variable_details_heading.set_content("Decision Logic (Context)");
        container.add_child(variable_details_heading);
      }
      ExpressionInstance::DecisionTable(decision_table) => {
        let mut variable_details_heading = HtmlElement::new_div(Some("variable-details-heading"));
        variable_details_heading.set_content("Decision Logic (Decision Table)");
        container.add_child(variable_details_heading);
        container.add_child(create_horizontal_decision_table_elements(decision_table));
      }
      ExpressionInstance::FunctionDefinition(_) => {
        let mut variable_details_heading = HtmlElement::new_div(Some("variable-details-heading"));
        variable_details_heading.set_content("Decision Logic (Function Definition)");
        container.add_child(variable_details_heading);
      }
      ExpressionInstance::Invocation(_) => {
        let mut variable_details_heading = HtmlElement::new_div(Some("variable-details-heading"));
        variable_details_heading.set_content("Decision Logic (Invocation)");
        container.add_child(variable_details_heading);
      }
      ExpressionInstance::List(_) => {
        let mut variable_details_heading = HtmlElement::new_div(Some("variable-details-heading"));
        variable_details_heading.set_content("Decision Logic (List)");
        container.add_child(variable_details_heading);
      }
      ExpressionInstance::LiteralExpression(literal_expression) => {
        let mut variable_details_heading = HtmlElement::new_div(Some("variable-details-heading"));
        variable_details_heading.set_content("Decision Logic (Literal Expression)");
        container.add_child(variable_details_heading);
        let mut element_literal_expression = HtmlElement::new_div(Some("literal-expression"));
        let no_literal_expression = "(no literal expression specified)".to_string();
        let text = literal_expression.text().as_ref().unwrap_or(&no_literal_expression);
        element_literal_expression.set_content(text);
        container.add_child(element_literal_expression);
      }
      ExpressionInstance::Relation(_) => {
        let mut variable_details_heading = HtmlElement::new_div(Some("variable-details-heading"));
        variable_details_heading.set_content("Decision Logic (Relation)");
        container.add_child(variable_details_heading);
      }
    }
    return Some(container);
  }
  None
}

/// Creates a description element enclosed in description container.
fn create_description_in_container(optional_description: &Option<String>) -> Option<HtmlElement> {
  if let Some(description_element) = create_description(optional_description) {
    let mut html_element = HtmlElement::new_div(CLASS_DESCRIPTION_CONTAINER);
    html_element.add_child(description_element);
    return Some(html_element);
  }
  None
}

/// Creates a description element.
fn create_description(optional_description: &Option<String>) -> Option<HtmlElement> {
  if let Some(description) = optional_description {
    let mut html_element = HtmlElement::new_div(CLASS_DESCRIPTION);
    html_element.set_content(&from_markdown(description));
    return Some(html_element);
  }
  None
}

/// Creates `SVG` shape representing decision.
fn create_svg_decision(shape: &DmnShape, decision: &Decision) -> HtmlElement {
  let mut rect = HtmlElement::new("rect");
  rect.set_attr("x", shape.bounds.x.to_string());
  rect.set_attr("y", shape.bounds.y);
  rect.set_attr("width", shape.bounds.width);
  rect.set_attr("height", shape.bounds.height);
  let shared_class_name = get_shared_class_name(shape);
  if let Some(shared_class) = &shared_class_name {
    rect.set_attr("class", shared_class);
  }
  let text = create_svg_text(&shape.bounds, shared_class_name, get_label_shared_class_name(shape), get_label_text(shape, decision.name()));
  create_svg_group(vec![rect, text])
}

/// Creates `SVG` shape representing input data.
fn create_svg_input_data(shape: &DmnShape, input_data: &InputData) -> HtmlElement {
  let radius = shape.bounds.height / 2.0;
  let mut rect = HtmlElement::new("rect");
  rect.set_attr("x", shape.bounds.x);
  rect.set_attr("y", shape.bounds.y);
  rect.set_attr("width", shape.bounds.width);
  rect.set_attr("height", shape.bounds.height);
  rect.set_attr("rx", radius);
  rect.set_attr("ry", radius);
  let shared_class_name = get_shared_class_name(shape);
  if let Some(shared_class) = &shared_class_name {
    rect.set_attr("class", shared_class);
  }
  let text = create_svg_text(
    &shape.bounds,
    shared_class_name,
    get_label_shared_class_name(shape),
    get_label_text(shape, input_data.name()),
  );
  create_svg_group(vec![rect, text])
}

/// Creates `SVG` shape representing business knowledge model.
pub fn create_svg_business_knowledge_model(shape: &DmnShape, bkm: &BusinessKnowledgeModel) -> HtmlElement {
  let (x, y, w, h) = (shape.bounds.x, shape.bounds.y, shape.bounds.width, shape.bounds.height);
  let points = format!(
    "{},{} {},{} {},{} {},{} {},{} {},{}",
    x,
    y + 15.0,
    x + 15.0,
    y,
    x + w,
    y,
    x + w,
    y + h - 15.0,
    x + w - 15.0,
    y + h,
    x,
    y + h
  );
  let mut polygon = HtmlElement::new("polygon");
  polygon.set_attr("points", points);
  let shared_class_name = get_shared_class_name(shape);
  if let Some(shared_class) = &shared_class_name {
    polygon.set_attr("class", shared_class);
  }
  let text = create_svg_text(&shape.bounds, shared_class_name, get_label_shared_class_name(shape), get_label_text(shape, bkm.name()));
  create_svg_group(vec![polygon, text])
}

/// Creates `SVG` shape representing knowledge source.
pub fn create_svg_knowledge_source(shape: &DmnShape, knowledge_source: &KnowledgeSource) -> HtmlElement {
  let path = get_path_to_knowledge_source(&shape.bounds);
  let mut svg_path = HtmlElement::new("path");
  svg_path.set_attr("d", path);
  let shared_class_name = get_shared_class_name(shape);
  if let Some(shared_class) = &shared_class_name {
    svg_path.set_attr("class", shared_class);
  }
  let bounds = DcBounds {
    height: shape.bounds.height - (AMPLITUDE / 2.0) - 5.0,
    ..shape.bounds
  };
  let svg_text = create_svg_text(
    &bounds,
    shared_class_name,
    get_label_shared_class_name(shape),
    get_label_text(shape, knowledge_source.name()),
  );
  create_svg_group(vec![svg_path, svg_text])
}

/// Creates `SVG` solid edge line with black-filled arrow at the end.  
fn create_svg_edge_solid_with_black_arrow(way_points: &[DcPoint]) -> HtmlElement {
  // prepare line
  let points = way_points.iter().fold("".to_string(), |acc, w| format!("{}{},{} ", acc, w.x, w.y));
  let mut svg_edge = HtmlElement::new("polyline");
  svg_edge.set_attr("points", points);
  svg_edge.set_attr("stroke", "black");
  // prepare arrow
  let start_point = &way_points[way_points.len() - 2];
  let end_point = &way_points[way_points.len() - 1];
  let points = format!(
    "{},{} {},{} {},{}",
    end_point.x,
    end_point.y,
    end_point.x + 12.0,
    end_point.y - 4.0,
    end_point.x + 12.0,
    end_point.y + 4.0
  );
  let angle = get_angle(start_point, end_point);
  let rotate = format!("rotate({},{},{})", angle, end_point.x, end_point.y);
  let mut svg_arrow = HtmlElement::new("polygon");
  svg_arrow.set_attr("points", points);
  svg_arrow.set_attr("transform", rotate);
  svg_arrow.set_attr("fill", "black");
  svg_arrow.set_attr("stroke", "none");
  // return a group of arrow elements
  create_svg_group(vec![svg_edge, svg_arrow])
}

/// Creates `SVG` dashed edge line with thin arrow at the end.  
fn create_svg_edge_dashed_with_thin_arrow(way_points: &[DcPoint]) -> HtmlElement {
  // prepare line
  let points = way_points.iter().fold("".to_string(), |acc, w| format!("{}{},{} ", acc, w.x, w.y));
  let mut svg_edge = HtmlElement::new("polyline");
  svg_edge.set_attr("points", points);
  svg_edge.set_attr("stroke-dasharray", "5 3");
  // prepare arrow
  let start_point = &way_points[way_points.len() - 2];
  let end_point = &way_points[way_points.len() - 1];
  let path = format!(
    "M {},{} l {},{} M {},{} l {}, {}",
    end_point.x, end_point.y, 12.0, -4.0, end_point.x, end_point.y, 12.0, 4.0
  );
  let angle = get_angle(start_point, end_point);
  let rotate = format!("rotate({},{},{})", angle, end_point.x, end_point.y);
  let mut svg_arrow = HtmlElement::new("path");
  svg_arrow.set_attr("d", path);
  svg_arrow.set_attr("transform", rotate);
  svg_arrow.set_attr("fill", "none");
  svg_arrow.set_attr("stroke", "black");
  // return a group of arrow elements
  create_svg_group(vec![svg_edge, svg_arrow])
}

/// Creates `SVG` dashed edge line with black end-point at the end.  
fn create_svg_edge_dashed_with_end_point(way_points: &[DcPoint]) -> HtmlElement {
  // prepare line
  let points = way_points.iter().fold("".to_string(), |acc, w| format!("{}{},{} ", acc, w.x, w.y));
  let mut svg_edge = HtmlElement::new("polyline");
  svg_edge.set_attr("points", points);
  svg_edge.set_attr("stroke-dasharray", "5 3");
  // prepare arrow (ending point)
  let end_point = &way_points[way_points.len() - 1];
  let mut svg_arrow = HtmlElement::new("circle");
  svg_arrow.set_attr("cx", end_point.x);
  svg_arrow.set_attr("cy", end_point.y);
  svg_arrow.set_attr("r", "4");
  svg_arrow.set_attr("fill", "black");
  svg_arrow.set_attr("stroke", "none");
  // return a group of arrow elements
  create_svg_group(vec![svg_edge, svg_arrow])
}

/// Prepares `SVG` object containing multiline text.
///
/// Text given in argument `text` is placed in the following construct:
///
/// ```text
/// <foreignObject>
///   <div>
///     <span>...text...</span>
///   </div>
/// </foreignObject>
/// ```
/// The `div` and `span` elements have such styles set, that the content is displayed as a table cell.
/// Text in a table cell is centered horizontally and aligned vertically in the middle.
///
fn create_svg_text(bounds: &DcBounds, parent_style: Option<String>, label_style: Option<String>, text: String) -> HtmlElement {
  // create span with content
  let mut span = HtmlElement::new("span");
  span.set_content(&text);
  // apply styles
  span.set_attr("style", "display:table-cell;vertical-align:middle;");
  // apply shared classes
  match (parent_style, label_style) {
    (Some(parent_class), Some(label_class)) => span.set_attr("class", format!("{} {}", label_class, parent_class)),
    (Some(parent_class), None) => span.set_attr("class", parent_class),
    (None, Some(label_class)) => span.set_attr("class", label_class),
    _ => {}
  }
  // create div
  let mut div = HtmlElement::new("div");
  div.set_attr("style", "display:table;height:100%;width:100%;text-align:center;");
  div.add_child(span);
  // create foreignObject
  let mut foreign_object = HtmlElement::new("foreignObject");
  foreign_object.set_attr("x", bounds.x + 4.0);
  foreign_object.set_attr("y", bounds.y);
  foreign_object.set_attr("width", bounds.width - 8.0);
  foreign_object.set_attr("height", bounds.height - 2.0);
  foreign_object.add_child(div);
  foreign_object
}

/// Returns the text of the label associated with the shape,
/// when no label is present then the specified name is returned.
fn get_label_text(shape: &DmnShape, name: &str) -> String {
  if let Some(label) = &shape.label {
    if let Some(label_text) = &label.text {
      return label_text.to_string();
    }
  }
  name.to_string()
}

/// Returns the identifier of the optional shared style.
fn get_shared_class_name(shape: &DmnShape) -> Option<String> {
  if let Some(shared_style_id) = &shape.shared_style {
    return Some(format!("{}{}", DIAGRAM_SHARED_STYLE_PREFIX, shared_style_id));
  }
  None
}

/// Returns the identifier of the optional shared style for label.
fn get_label_shared_class_name(shape: &DmnShape) -> Option<String> {
  if let Some(label) = &shape.label {
    if let Some(shared_style_id) = &label.shared_style {
      return Some(format!("{}{}", DIAGRAM_SHARED_STYLE_PREFIX, shared_style_id));
    }
  }
  None
}

/// Returns the rotation angle of an arrow.
fn get_angle(start: &DcPoint, end: &DcPoint) -> f64 {
  let x = end.x - start.x;
  let y = end.y - start.y;
  if x == 0.0 {
    return if y >= 0.0 { -90.0 } else { 90.0 };
  }
  let angle = ((y / x).atan() * 360.0) / PI_2;
  if x > 0.0 {
    if y >= 0.0 {
      angle - 180.0
    } else {
      angle + 180.0
    }
  } else {
    angle
  }
}

///
fn get_path_to_knowledge_source(bounds: &DcBounds) -> String {
  let period_div_2 = AMPLITUDE / 2.0;
  let curve_base_height = bounds.y + bounds.height - period_div_2;
  let width_div_4: f64 = bounds.width / 4.0;
  let width_div_2: f64 = bounds.width / 2.0;

  let mut path = format!("M {} {}", bounds.x, bounds.y);
  path = format!("{} L {} {}", path, bounds.x + bounds.width, bounds.y);
  path = format!("{} L {} {}", path, bounds.x + bounds.width, curve_base_height);
  path = format!(
    "{} C {},{} {},{} {},{}",
    path,
    bounds.x + bounds.width,
    curve_base_height,
    bounds.x + bounds.width - width_div_4,
    curve_base_height - AMPLITUDE,
    bounds.x + bounds.width - width_div_2,
    curve_base_height
  );
  path = format!(
    "{} C {},{} {},{} {},{}",
    path,
    bounds.x + bounds.width - width_div_2,
    curve_base_height,
    bounds.x + width_div_4,
    curve_base_height + AMPLITUDE,
    bounds.x,
    curve_base_height
  );
  path = format!("{} L {} {} Z", path, bounds.x, bounds.y);
  path
}

/// Creates `<svg>` tag with specified dimension and content.
pub fn create_svg_tag(title: &str, dimension: &Option<DcDimension>, elements: Vec<HtmlElement>) -> HtmlElement {
  let mut svg = HtmlElement::new("svg");
  if let Some(size) = dimension {
    let width = size.width.ceil();
    let height = size.height.ceil();
    svg.set_attr("viewBox", format!("0 0 {width} {height}"));
    svg.set_attr("width", width.to_string());
  }
  for element in elements {
    svg.add_child(element);
  }
  let mut diagram_title = HtmlElement::new_div(CLASS_DIAGRAM_TITLE);
  diagram_title.set_content(title);
  let mut diagram_container = HtmlElement::new_div(CLASS_DIAGRAM_CONTAINER);
  diagram_container.add_child(diagram_title);
  diagram_container.add_child(svg);
  diagram_container
}

/// Creates `<g>` tag containing specified elements.
fn create_svg_group(elements: Vec<HtmlElement>) -> HtmlElement {
  let mut group = HtmlElement::new("g");
  for element in elements {
    group.add_child(element);
  }
  group
}

/// Creates `HTML` heading tag with specified level and content.
fn create_html_heading(level: HeadingLevel, content: &str) -> HtmlElement {
  let tag_name = match level {
    HeadingLevel::H1 => "h1",
    HeadingLevel::H2 => "h2",
    HeadingLevel::H3 => "h3",
    HeadingLevel::H4 => "h4",
    HeadingLevel::H5 => "h5",
    HeadingLevel::H6 => "h6",
  };
  let mut heading = HtmlElement::new(tag_name);
  heading.set_content(content);
  heading
}

/// Converts markdown content into HTML content.
fn from_markdown(input: &str) -> String {
  let trimmed_input = input.lines().map(|line| line.trim().to_string()).collect::<Vec<String>>().join("\n");
  markdown::to_html(&trimmed_input)
    .trim()
    .replace("&lt;", "<")
    .replace("&gt;", ">")
    .replace("&amp;", "&")
    .replace("&quot;", "\"")
    .replace("&apos;", "\'")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_html_heading() {
    assert_eq!("<h1>Heading H1</h1>", create_html_heading(HeadingLevel::H1, "Heading H1").to_string());
    assert_eq!("<h2>Heading H2</h2>", create_html_heading(HeadingLevel::H2, "Heading H2").to_string());
    assert_eq!("<h3>Heading H3</h3>", create_html_heading(HeadingLevel::H3, "Heading H3").to_string());
    assert_eq!("<h4>Heading H4</h4>", create_html_heading(HeadingLevel::H4, "Heading H4").to_string());
    assert_eq!("<h5>Heading H5</h5>", create_html_heading(HeadingLevel::H5, "Heading H5").to_string());
    assert_eq!("<h6>Heading H6</h6>", create_html_heading(HeadingLevel::H6, "Heading H6").to_string());
  }
}
