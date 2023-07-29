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

use super::*;

from_examples!(DMN_3_0098);

static_context!(CTX, r#"{}"#);

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_001", &CTX, r#"38"#);
}

#[test]
fn _0002() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_002", &CTX, r#"1"#);
}

#[test]
fn _0003() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_003", &CTX, r#"1"#);
}

#[test]
fn _0004() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_004", &CTX, r#"1"#);
}

#[test]
fn _0005() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_005", &CTX, r#"53"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{date_input_001: @"1970-01-01"}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_006", &ctx, r#"1"#);
}

#[test]
fn _0007() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_007", &CTX, r#"38"#);
}

#[test]
fn _0008() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_008", &CTX, r#"[53, 1, 1]"#);
}

#[test]
fn _0009() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "datetime_001", &CTX, r#"1"#);
}

#[test]
fn _0010() {
  let ctx = context(r#"{date_input_001: @"1970-01-01T10:10:10"}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "datetime_004", &ctx, r#"1"#);
}

#[test]
fn _0011() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "datetime_005", &CTX, r#"38"#);
}

#[test]
fn _0012() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "null_001",
    &CTX,
    r#"null([core::week of year] invalid argument type, expected date, date and time, actual type is Null)"#,
  );
}

#[test]
fn _0013() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "null_002",
    &CTX,
    r#"null(expected 1 parameters, actual number of parameters is 0)"#,
  );
}

#[test]
fn _0014() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "null_003",
    &CTX,
    r#"null([core::week of year] invalid argument type, expected date, date and time, actual type is string)"#,
  );
}

#[test]
fn _0015() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "null_004",
    &CTX,
    r#"null([core::week of year] invalid argument type, expected date, date and time, actual type is Null)"#,
  );
}

#[test]
fn _0016() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "null_005",
    &CTX,
    r#"null([core::week of year] invalid argument type, expected date, date and time, actual type is string)"#,
  );
}

#[test]
fn _0017() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "null_006", &CTX, r#"null(parameter 'date' not found)"#);
}

#[test]
fn _0018() {
  let ctx = context(r#"{date_input_001: "foo"}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "null_007",
    &ctx,
    r#"null([core::week of year] invalid argument type, expected date, date and time, actual type is string)"#,
  );
}

#[test]
fn _0019() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "null_008",
    &CTX,
    r#"null(expected 1 parameters, actual number of parameters is 2)"#,
  );
}
