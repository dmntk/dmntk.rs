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

from_examples!(DMN_3_0085);

#[bench]
fn _0001(b: &mut Bencher) {
  let input_data = r#"{}"#;
  let ctx = context(input_data);
  let invocable_name = "decisionService_001";
  assert_decision_service(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, input_data, r#""foo""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let input_data = r#"{decision_002_input: "baz"}"#;
  let ctx = context(input_data);
  let invocable_name = "decisionService_002";
  assert_decision_service(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, input_data, r#""foo baz""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0002_a(b: &mut Bencher) {
  let input_data = r#"{}"#;
  let ctx = context(input_data);
  let invocable_name = "decisionService_002";
  assert_decision_service(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    input_data,
    r#"null(expected string as a second argument in addition)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0002_b(b: &mut Bencher) {
  let input_data = r#"{decision_002_input: null}"#;
  let ctx = context(input_data);
  let invocable_name = "decisionService_002";
  assert_decision_service(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    input_data,
    r#"null(expected string as a second argument in addition)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0002_c(b: &mut Bencher) {
  let input_data = r#"{decision_002_input: 1234}"#;
  let ctx = context(input_data);
  let invocable_name = "decisionService_002";
  assert_decision_service(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    input_data,
    r#"null(expected string as a second argument in addition)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let input_data = r#"{decision_003_input_1: "B", decision_003_input_2: "C", inputData_003: "D"}"#;
  let ctx = context(input_data);
  let invocable_name = "decisionService_003";
  assert_decision_service(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, input_data, r#""A B C D""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision_004_1";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#""foo""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let input_data = r#"{}"#;
  let ctx = context(input_data);
  let invocable_name = "decisionService_005";
  assert_decision_service(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, input_data, r#""foo""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let input_data = r#"{}"#;
  let ctx = context(input_data);
  let invocable_name = "decision_005_1";
  assert_decision_service(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, input_data, r#"null(invalid number of arguments)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0007(b: &mut Bencher) {
  let input_data = r#"{}"#;
  let ctx = context(input_data);
  let invocable_name = "decision_005_2";
  assert_decision_service(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, input_data, r#""foo""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}
