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

#![feature(test)]

extern crate test;

use dmntk_feel::values::Value;
use dmntk_feel::{scope, value_null, value_number, FeelNumber, FeelScope};
use dmntk_feel_evaluator::{prepare, BuildContext};
use test::Bencher;

#[bench]
fn feel_evaluator_addition_0001(b: &mut Bencher) {
    let scope = scope!();
    let input = r#"1+2"#;
    let node = dmntk_feel_parser::parse_expression(&scope, input, false).unwrap();
    let evaluator = prepare(&BuildContext::default(), &node).unwrap();
    assert_eq!(value_number!(3), evaluator(&scope));
    b.iter(|| evaluator(&scope));
}

#[bench]
fn feel_evaluator_addition_0002(b: &mut Bencher) {
    let scope = scope!();
    let input = r#"1+2+3"#;
    let node = dmntk_feel_parser::parse_expression(&scope, input, false).unwrap();
    let evaluator = prepare(&BuildContext::default(), &node).unwrap();
    assert_eq!(value_number!(6), evaluator(&scope));
    b.iter(|| evaluator(&scope));
}

#[bench]
fn feel_evaluator_addition_0003(b: &mut Bencher) {
    let scope = scope!();
    scope.set_value(&"a".into(), value_null!());
    scope.set_value(&"b".into(), value_null!());
    let input = r#"a+b"#;
    let node = dmntk_feel_parser::parse_expression(&scope, input, false).unwrap();
    let evaluator = prepare(&BuildContext::default(), &node).unwrap();
    scope.set_value(&"a".into(), value_number!(1838, 2));
    scope.set_value(&"b".into(), value_number!(162, 2));
    assert_eq!(value_number!(20), evaluator(&scope));
    b.iter(|| evaluator(&scope));
}
