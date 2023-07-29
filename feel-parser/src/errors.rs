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

//! Implementation of errors for FEEL parser.

use dmntk_common::{DmntkError, ToErrorMessage};

/// Lexer error.
#[derive(ToErrorMessage)]
struct LexerError(String);

pub fn err_unexpected_eof() -> DmntkError {
  LexerError("unexpected end of file".to_string()).into()
}

pub fn err_expected_hex_digit(ch: char) -> DmntkError {
  LexerError(format!("expected hex digit but encountered '{ch}'")).into()
}

pub fn err_unicode_value_out_of_range(value: u64) -> DmntkError {
  LexerError(format!("value is out of allowed Unicode range 0x0000..0x10FFFF : {value:X}")).into()
}

pub fn err_unicode_surrogate_out_of_range(value: u64) -> DmntkError {
  LexerError(format!("surrogate value is out of allowed range 0xD800..0xDFFF : {value:X}")).into()
}

/// Parser error.
#[derive(ToErrorMessage)]
struct ParserError(String);

/// Creates an error when `FEEL` name was expected on input, but something else encountered.
pub fn err_not_a_feel_name(s: &str) -> DmntkError {
  ParserError(format!("expected `FEEL` name on input but found `{s}`")).into()
}

/// Creates syntax error on specified input.
pub fn err_syntax_error(input: &str) -> DmntkError {
  ParserError(format!("syntax error: {input}")).into()
}
