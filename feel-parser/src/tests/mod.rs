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

mod ast;
mod bifs;
mod closure;
mod context;
mod expr;

/// Creates a parsing scope.
macro_rules! scope {
  () => {{
    use crate::tests::ParsingScope;
    ParsingScope::default()
  }};
}

use crate::lalr::TokenType;
use crate::lalr::TokenType::StartExpression;
use crate::parser::Parser;
use crate::ParsingScope;
use difference::Changeset;
use dmntk_feel::Name;
pub(crate) use scope;

/// Parses the input text and compared the result with expected value.
fn accept(scope: &ParsingScope, start_token_type: TokenType, input: &str, expected: &str, trace: bool) {
  let node = Parser::new(scope, start_token_type, input, trace).parse().unwrap();
  let actual = node.to_string();
  if actual != expected {
    println!("EXPECTED:\n------------------------------------------------------------{expected}\n");
    println!("ACTUAL:\n------------------------------------------------------------{actual}\n");
    println!(
      "DIFF:\n------------------------------------------------------------{}\n",
      Changeset::new(expected, &actual, "")
    );
  }
  assert_eq!(expected, actual);
}

#[test]
fn test_parse_textual_expression() {
  let scope = dmntk_feel::FeelScope::default();
  let node = crate::parse_textual_expression(&scope, "1+2", false).unwrap();
  assert_eq!(
    r#"
       Add
       ├─ Numeric
       │  └─ `1.`
       └─ Numeric
          └─ `2.`
    "#,
    node.to_string()
  );
}

#[test]
fn test_parse_textual_expressions() {
  let scope = dmntk_feel::FeelScope::default();
  let node = crate::parse_textual_expressions(&scope, "1+2,2+3,3*4", false).unwrap();
  assert_eq!(
    r#"
       ExpressionList
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `1.`
       │  └─ Numeric
       │     └─ `2.`
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `2.`
       │  └─ Numeric
       │     └─ `3.`
       └─ Mul
          ├─ Numeric
          │  └─ `3.`
          └─ Numeric
             └─ `4.`
    "#,
    node.to_string()
  );
}

#[test]
fn test_parse_unary_tests() {
  let scope = dmntk_feel::FeelScope::default();
  let node = crate::parse_unary_tests(&scope, "1,2,3,4", false).unwrap();
  assert_eq!(
    r#"
       ExpressionList
       ├─ Numeric
       │  └─ `1.`
       ├─ Numeric
       │  └─ `2.`
       ├─ Numeric
       │  └─ `3.`
       └─ Numeric
          └─ `4.`
    "#,
    node.to_string()
  );
}

#[test]
fn test_parse_boxed_expression() {
  let scope = dmntk_feel::FeelScope::default();
  let node = crate::parse_boxed_expression(&scope, "[1,2,3,4]", false).unwrap();
  assert_eq!(
    r#"
       List
       ├─ Numeric
       │  └─ `1.`
       ├─ Numeric
       │  └─ `2.`
       ├─ Numeric
       │  └─ `3.`
       └─ Numeric
          └─ `4.`
    "#,
    node.to_string()
  );
}

#[test]
fn test_parse_context() {
  let scope = dmntk_feel::FeelScope::default();
  let node = crate::parse_context(&scope, "{age: 50}", false).unwrap();

  assert_eq!(
    r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `age`
          └─ Numeric
             └─ `50.`
    "#,
    node.to_string()
  );
}

#[test]
fn test_parse_name() {
  let name_a: Name = Name::new(&["Full", "House"]);
  let scope = dmntk_feel::FeelScope::default();
  assert_eq!(name_a, crate::parse_name(&scope, "Full House", false).unwrap());
}

#[test]
fn test_parse_longest_name() {
  let name_a: Name = Name::new(&["Full", "House", "With", "A", "Cat"]);
  assert_eq!(name_a, crate::parse_longest_name(" Full House  With  \t A \n\n Cat    ").unwrap());
}

/// Covers the case when function `accept` reports an error,
/// which means that the test result differs from expected value.
#[test]
#[should_panic]
fn test_not_accept() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "1+2",
    r#"
        Add
          Numeric 1.
          Numeric 3.
      "#,
    false,
  );
}
