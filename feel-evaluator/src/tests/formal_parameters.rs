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

use super::*;
use dmntk_feel::{scope, FeelType};

#[test]
fn _0001() {
  let node = AstNode::FormalParameter(Box::new(AstNode::ParameterName("n".into())), Box::new(AstNode::FeelType(FeelType::Boolean)));
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!("FormalParameter", result.ok().unwrap().to_string());
}

#[test]
fn _0002() {
  let node = AstNode::FormalParameter(Box::new(AstNode::Name("n".into())), Box::new(AstNode::FeelType(FeelType::Boolean)));
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!("null(expected name of the formal parameter)", result.ok().unwrap().to_string());
}

#[test]
fn _0003() {
  let parameter_x = AstNode::FormalParameter(Box::new(AstNode::ParameterName("x".into())), Box::new(AstNode::FeelType(FeelType::Number)));
  let parameter_y = AstNode::FormalParameter(Box::new(AstNode::ParameterName("y".into())), Box::new(AstNode::FeelType(FeelType::Number)));
  let node = AstNode::FormalParameters(vec![parameter_x, parameter_y]);
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!("FormalParameters", result.ok().unwrap().to_string());
}

#[test]
fn _0004() {
  let parameter_x = AstNode::FormalParameter(Box::new(AstNode::ParameterName("x".into())), Box::new(AstNode::FeelType(FeelType::Number)));
  let parameter_y = AstNode::FormalParameter(Box::new(AstNode::ParameterName("y".into())), Box::new(AstNode::FeelType(FeelType::Number)));
  let parameter_invalid = AstNode::Numeric("1".into(), "0".into());
  let node = AstNode::FormalParameters(vec![parameter_x, parameter_y, parameter_invalid]);
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!("null(expected formal parameter, actual value type is: number)", result.ok().unwrap().to_string());
}
