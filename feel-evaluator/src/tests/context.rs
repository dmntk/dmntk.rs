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
use dmntk_feel::scope;

#[test]
fn _0001() {
  let input = r#"
  {
    Full Name: "John Doe",
    Full Name: "John Travis"
  }
  "#;
  let node = dmntk_feel_parser::parse_context(&scope!(), input, false).unwrap();
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"null(duplicated context entry key: Full Name)"#, result.ok().unwrap().to_string());
}

#[test]
fn _0002() {
  let input = r#"
  {
    "Full Name": "John Doe",
    "Address": "Atlanta"
  }
  "#;
  let node = dmntk_feel_parser::parse_context(&scope!(), input, false).unwrap();
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"{Address: "Atlanta", Full Name: "John Doe"}"#, result.ok().unwrap().to_string());
}

#[test]
fn _0003() {
  let node = AstNode::Context(vec![AstNode::String("key".to_string())]);
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"null(expected context entry, actual value type is string)"#, result.ok().unwrap().to_string());
}

#[test]
fn _0004() {
  let node = AstNode::Context(vec![AstNode::ContextEntry(
    Box::new(AstNode::String("key".to_string())),
    Box::new(AstNode::String("value".to_string())),
  )]);
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"null(expected context entry key, actual value type is string)"#, result.ok().unwrap().to_string());
}

#[test]
fn _0005() {
  let input = r#" { convert: function(x: context <age: number, name: stringa>) x } "#;
  let node = dmntk_feel_parser::parse_context(&scope!(), input, false).unwrap();
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"{convert: null(expected type of the formal parameter)}"#, result.ok().unwrap().to_string());
}

#[test]
fn _0006() {
  let node = AstNode::ContextType(vec![AstNode::ContextTypeEntry(
    Box::new(AstNode::String("key".to_string())),
    Box::new(AstNode::String("value".to_string())),
  )]);
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"null(expected a name in context type entry)"#, result.ok().unwrap().to_string());
}

#[test]
fn _0007() {
  let node = AstNode::ContextType(vec![AstNode::String("key".to_string())]);
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"null(expected context type entry, actual value type is string)"#, result.ok().unwrap().to_string());
}
