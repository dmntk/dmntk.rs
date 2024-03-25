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

//! Testing parsing of invalid statements.

use super::super::*;
use crate::lalr::TokenType::StartExpression;
use crate::parser::Parser;
use dmntk_common::DmntkError;
use std::str::from_utf8;

/// Utility function to shorten repeatable code in tests.
fn te(input: &str, source: &str, message: &str) {
  assert_eq!(Err(DmntkError::new(source, message)), Parser::new(&scope!(), StartExpression, input, false).parse())
}

#[test]
fn _0001() {
  // Caret character is not valid at the beginning of any FEEL statement.
  te(r#"^123"#, "ParserError", r#"syntax error: ^123"#);
}

#[test]
fn _0002() {
  // Vertical space is not allowed inside FEEL string.
  let input = from_utf8(&[34, 49, 50, 10, 51, 34]).unwrap(); // "12\n3"
  te(input, "ParserError", "syntax error: \"12\n3\"");
}

#[test]
fn _0003() {
  // Unexpected end of input before the FEEL string is closed with quotation mark `"`.
  te(r#""123"#, "ParserError", "syntax error: \"123");
}

#[test]
fn _0004() {
  // Unexpected end of input when parsing hex digits.
  te(r#""a\u4B"#, "LexerError", "unexpected end of file");
}

#[test]
fn _0005() {
  // After decimal point must always be a digit.
  te(r#"1. + 2"#, "ParserError", "syntax error: 1. + 2");
}

#[test]
fn _0006() {
  // After decimal point must be a digit.
  te(r#"1.^ + 2"#, "ParserError", "syntax error: 1.^ + 2");
}

#[test]
fn _0007() {
  // End of input after decimal point.
  te(r#"1."#, "ParserError", "syntax error: 1.");
}

#[test]
fn _0008() {
  // Keyword `function` interpreted as a name. This is ok, when such name is in scope.
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "function ",
    r#"
       Name
       └─ `function`
    "#,
    false,
  );
}

#[test]
fn _0009() {
  // Invalid UTF-16 surrogate.
  te(r#""\uD800\uE000""#, "LexerError", "surrogate value is out of allowed range 0xD800..0xDFFF : E000");
}

#[test]
fn _0010() {
  // Invalid UTF value.
  te(r#""\U110000""#, "LexerError", "value is out of allowed Unicode range 0x0000..0x10FFFF : 110000");
}

#[test]
fn _0011() {
  let scope = scope!();
  assert_eq!(
    "<ParserError> syntax error: 1 += 2",
    Parser::new(&scope, StartExpression, "1 += 2", false).parse().err().unwrap().to_string().as_str()
  );
}
