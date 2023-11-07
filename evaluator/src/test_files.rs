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

//! Implementation of the evaluator for test files.
//!
//! Test files (textual files containing test cases) are provided for automated testing
//! the **dmntk** as a black-box.
//!
//! Test files are used in evaluation tests for:
//! - `FEEL` expressions (see `tfe` subcommand),
//! - `DMN` models (see `tmd` subcommand),
//! - decision tables (see `tdt` subcommand).
//!
//! Single test case has the following structure:
//! ```text
//! separator input_data , expected_result
//! ```
//! where:
//! - **separator** is one or more separator characters: `~`, `!`, `@`, `#`, `$`, `%`, `^`, `&`, `|`,
//! - **input_data** is a valid `FEEL` context containing input data for a test case,
//! - **,** is literally the comma character,
//! - **expected_result** is a valid `FEEL` value that is expected as a result in test case.
//!
//! Test file may contain one or more test cases.
//!
//! # Example
//!
//! An example of a test file may look like this:
//! ```text
//! % { Customer:"Business",   Order:  -3.23 }, 0.10
//! % { Customer:"Business",   Order:   9.00 }, 0.10
//! % { Customer:"Business",   Order:  10.00 }, 0.15
//! % { Customer:"Business",   Order: 120.00 }, 0.15
//! % { Customer:"Private",    Order:  -2.34 }, 0.05
//! % { Customer:"Private",    Order:  10.00 }, 0.05
//! % { Customer:"Private",    Order: 101.00 }, 0.05
//! % { Customer:"Government", Order:  10.00 }, null
//! ```

use dmntk_common_1::{DmntkError, Result};
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::FeelScope;
use dmntk_feel_parser::AstNode;

/// Evaluates test cases loaded from input test file.
pub fn evaluate_test_cases(input: &str) -> Result<Vec<(FeelContext, Value)>> {
    let mut test_cases = vec![];
    if let Some(separator) = detect_separator(input) {
        let scope = FeelScope::default();
        for unary_tests in split_test_cases(input, &separator) {
            match dmntk_feel_parser::parse_unary_tests(&scope, unary_tests, false) {
                Ok(ast_node) => match ast_node {
                    AstNode::ExpressionList(nodes) => {
                        if nodes.len() == 2 {
                            if let Ok(input_data) =
                                dmntk_feel_evaluator::evaluate_context_node(&scope, &nodes[0])
                            {
                                if let Ok(expected_result) =
                                    dmntk_feel_evaluator::evaluate(&scope, &nodes[1])
                                {
                                    test_cases.push((input_data, expected_result));
                                }
                            }
                        }
                    }
                    other => {
                        return Err(DmntkError::new(
                            "Evaluator",
                            &format!("expected expression list, but found '{other}'"),
                        ))
                    }
                },
                Err(reason) => return Err(reason),
            }
        }
    }
    Ok(test_cases)
}

/// Splits test cases from input test file using specified separator.
fn split_test_cases<'a>(input: &'a str, separator: &'a str) -> Vec<&'a str> {
    let split = input.split(&separator);
    split
        .filter_map(|s| {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                Some(trimmed)
            } else {
                None
            }
        })
        .collect()
}

/// Detects the separator used in test file.
fn detect_separator(input: &str) -> Option<String> {
    let mut separator = String::with_capacity(80);
    let mut found = false;
    for ch in input.chars() {
        if found {
            if is_separator_character(ch) {
                separator.push(ch);
            } else {
                break;
            }
        } else if is_separator_character(ch) {
            separator.push(ch);
            found = true;
        }
    }
    if found {
        Some(separator)
    } else {
        None
    }
}

/// Returns `true` when specified character is a test case separator character.
///
/// Valid separator characters are: `~` , `!` , `@` , `#` , `$` , `%` , `^` , `&` , `|`.
///
fn is_separator_character(ch: char) -> bool {
    matches!(ch, '~' | '!' | '@' | '#' | '$' | '%' | '^' | '&' | '|')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_empty_test_file() {
        assert_eq!(0, evaluate_test_cases("").unwrap().len())
    }

    #[test]
    fn test_evaluate_test_file() {
        let input = r#"
      % { Customer:"Business",   Order:  -3.23 }, 0.10
      % { Customer:"Business",   Order:   9.00 }, 0.10
      % { Customer:"Business",   Order:  10.00 }, 0.15
      % { Customer:"Business",   Order: 120.00 }, 0.15
      % { Customer:"Private",    Order:  -2.34 }, 0.05
      % { Customer:"Private",    Order:  10.00 }, 0.05
      % { Customer:"Private",    Order: 101.00 }, 0.05
      % { Customer:"Government", Order:  10.00 }, null
    "#;
        assert_eq!(8, evaluate_test_cases(input).unwrap().len())
    }

    #[test]
    #[should_panic(
        expected = r#"DmntkError("<Evaluator> expected expression list, but found '\n       Irrelevant\n    '"#
    )]
    fn test_evaluate_invalid_test_file() {
        let input = r#"
      % -
    "#;
        evaluate_test_cases(input).unwrap();
    }

    #[test]
    fn test_detect_separator() {
        assert_eq!(None, detect_separator(""));
        assert_eq!("~", detect_separator("~").unwrap());
        assert_eq!("!", detect_separator("!").unwrap());
        assert_eq!("@", detect_separator("@").unwrap());
        assert_eq!("#", detect_separator("#").unwrap());
        assert_eq!("$", detect_separator("$").unwrap());
        assert_eq!("%", detect_separator("%").unwrap());
        assert_eq!("^", detect_separator("^").unwrap());
        assert_eq!("&", detect_separator("&").unwrap());
        assert_eq!("|", detect_separator("|").unwrap());
        assert_eq!("%%", detect_separator("   %%   ").unwrap());
        assert_eq!("%%", detect_separator(" \n\n\n \t  %%{   ").unwrap());
        assert_eq!("%%", detect_separator(" \n %%\n").unwrap());
        assert_eq!("~!@#$%^&|", detect_separator(" ~!@#$%^&| ").unwrap());
    }
}
