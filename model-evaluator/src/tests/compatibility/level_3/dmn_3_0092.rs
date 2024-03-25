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

from_examples!(DMN_3_0092);

static_context!(CTX, r#"{}"#);

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_001_1", &CTX, r#"3"#);
}

#[test]
fn _0002() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_002_1", &CTX, r#"4"#);
}

#[test]
fn _0003() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_003_1", &CTX, r#"5"#);
}

#[test]
fn _0004() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_004_1", &CTX, r#"6"#);
}

#[test]
fn _0005() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_005_1", &CTX, r#"20"#);
}

#[test]
fn _0006() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_006_1", &CTX, r#"30"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{input_007_1: 20}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_007_1", &ctx, r#"100"#);
}

#[test]
fn _0008() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_008_1", &CTX, r#"6"#);
}

#[test]
fn _0009() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_009_1", &CTX, r#"200"#);
}

#[test]
fn _0010() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_010_1", &CTX, r#"120"#);
}

#[test]
fn _0011() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_010_1_a", &CTX, r#"120"#);
}

#[test]
fn _0012() {
  let ctx = context(r#"{input_011_1: 10}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_011_1", &ctx, r#"5000"#);
}

#[test]
fn _0013() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_012_1", &CTX, r#"5000"#);
}

#[test]
fn _0014() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_013_1", &CTX, r#"5000"#);
}

#[test]
fn _0015() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_014_1", &CTX, r#"25"#);
}

#[test]
fn _0016() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_015_1", &CTX, r#"18"#);
}

#[test]
fn _0017() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "decision_016_1",
    &CTX,
    r#"null(expected 1 parameters, actual number of parameters is 2)"#,
  );
}

#[test]
fn _0018() {
  let ctx = context(r#"{input_017_1: "a"}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_017_1", &ctx, r#"["a", "a", "z", "z"]"#);
}

#[test]
fn _0019() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision_018", &CTX, r#"["a", "a", "z", "z"]"#);
}
