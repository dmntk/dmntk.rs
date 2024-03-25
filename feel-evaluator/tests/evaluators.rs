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

use dmntk_feel::values::Value;
use dmntk_feel::{FeelNumber, FeelScope};
use dmntk_feel_evaluator::{evaluate, evaluate_context, evaluate_context_node, evaluate_max, evaluate_min, evaluate_sum, prepare, BuildContext};
use dmntk_feel_parser::AstNode;

#[test]
fn _0001() {
  let scope = FeelScope::default();
  assert_eq!("{}", evaluate_context(&scope, r#"{}"#).unwrap().to_string());
}

#[test]
fn _0002() {
  let scope = FeelScope::default();
  let node = AstNode::Context(vec![AstNode::ContextEntry(
    Box::new(AstNode::ContextEntryKey("alpha".into())),
    Box::new(AstNode::Boolean(true)),
  )]);
  assert_eq!("{alpha: true}", evaluate_context_node(&scope, &node).unwrap().to_string());
}

#[test]
fn _0003() {
  let scope = FeelScope::default();
  let node = AstNode::Boolean(true);
  assert_eq!(
    "<FeelEvaluatorError> expected FEEL context as an input",
    evaluate_context_node(&scope, &node).err().unwrap().to_string()
  );
}

#[test]
fn _0004() {
  let scope = FeelScope::default();
  let node = AstNode::Add(
    Box::new(AstNode::Numeric("1".to_string(), "23".to_string())),
    Box::new(AstNode::Numeric("1".to_string(), "77".to_string())),
  );
  let evaluator = prepare(&BuildContext::default(), &node).unwrap();
  let value = evaluator(&scope) as Value;
  assert_eq!("3", value.to_string());
}

#[test]
fn _0005() {
  let value = evaluate_sum(vec![Value::Number(FeelNumber::new(123, 2)), Value::Number(FeelNumber::new(177, 2))]);
  assert_eq!("3", value.to_string());
}

#[test]
fn _0006() {
  let value = evaluate_min(vec![Value::Number(FeelNumber::new(123, 2)), Value::Number(FeelNumber::new(177, 2))]);
  assert_eq!("1.23", value.to_string());
}

#[test]
fn _0007() {
  let value = evaluate_max(vec![Value::Number(FeelNumber::new(123, 2)), Value::Number(FeelNumber::new(177, 2))]);
  assert_eq!("1.77", value.to_string());
}

#[test]
fn _0008() {
  let scope = FeelScope::default();
  let node = AstNode::FunctionInvocation(Box::new(AstNode::Name("calculate".into())), Box::new(AstNode::Boolean(true)));
  assert_eq!(
    "<FeelEvaluatorError> expected positional or named parameter",
    evaluate(&scope, &node).err().unwrap().to_string()
  );
}

#[test]
fn _0009() {
  let scope = FeelScope::default();
  let node = AstNode::FunctionInvocation(
    Box::new(AstNode::Name("calculate".into())),
    Box::new(AstNode::NamedParameters(vec![AstNode::NamedParameter(
      Box::new(AstNode::Boolean(true)),
      Box::new(AstNode::Boolean(false)),
    )])),
  );
  assert_eq!(
    "<FeelEvaluatorError> expected AstNode::ParameterName, actual node is Boolean(true)",
    evaluate(&scope, &node).err().unwrap().to_string()
  );
}

#[test]
fn _0010() {
  let scope = FeelScope::default();
  let node = AstNode::Every(Box::new(AstNode::Boolean(true)), Box::new(AstNode::Boolean(false)));
  assert_eq!(
    "<FeelEvaluatorError> expected AST node AstNode::QuantifiedContexts, actual AST node is Boolean(true)",
    evaluate(&scope, &node).err().unwrap().to_string()
  );
}

#[test]
fn _0011() {
  let scope = FeelScope::default();
  let node = AstNode::Some(Box::new(AstNode::Boolean(true)), Box::new(AstNode::Boolean(false)));
  assert_eq!(
    "<FeelEvaluatorError> expected AST node AstNode::QuantifiedContexts, actual AST node is Boolean(true)",
    evaluate(&scope, &node).err().unwrap().to_string()
  );
}

#[test]
fn _0012() {
  let scope = FeelScope::default();
  let node = AstNode::CommaList(vec![]);
  assert_eq!(
    "<FeelEvaluatorError> unexpected AST node in evaluator builder CommaList([])",
    evaluate(&scope, &node).err().unwrap().to_string()
  );
}
