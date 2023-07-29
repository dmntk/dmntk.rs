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

from_examples!(DMN_3_0076);
static CTX: Lazy<FeelContext> = Lazy::new(|| context(r#"{}"#));

#[bench]
fn _0001(b: &mut Bencher) {
  let invocable_name = "boxed_001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"456"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let invocable_name = "incorrect_001";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &CTX,
    r#"null(java.lang.NoSuchMethodException: java.lang.Math.foo(double))"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let invocable_name = "incorrect_002";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &CTX,
    r#"null(java.lang.ClassNotFoundException: java.lang.Foo)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let invocable_name = "incorrect_003";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &CTX,
    r#"null(java.lang.NoSuchMethodException: java.lang.Math.max(java.lang.String,java.lang.String))"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let invocable_name = "literal_001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"-0.8879689066918555"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let invocable_name = "literal_002";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"456.78"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0007(b: &mut Bencher) {
  let invocable_name = "literal_003";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"456"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0008(b: &mut Bencher) {
  let invocable_name = "literal_004";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"456"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0009(b: &mut Bencher) {
  let invocable_name = "literal_005";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"123"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0010(b: &mut Bencher) {
  let invocable_name = "literal_006";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"3"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0011(b: &mut Bencher) {
  let invocable_name = "literal_007";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#""a""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0012(b: &mut Bencher) {
  let invocable_name = "literal_007_a";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &CTX,
    r#"null(simple DTO conversion to object failed, class: char, type: XSD_STRING)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0013(b: &mut Bencher) {
  let invocable_name = "literal_008";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"456"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0014(b: &mut Bencher) {
  let invocable_name = "literal_009";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"456.78"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0015(b: &mut Bencher) {
  let invocable_name = "literal_010";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"123"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0016(b: &mut Bencher) {
  let invocable_name = "literal_011";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"1234.56"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0017(b: &mut Bencher) {
  let invocable_name = "literal_012";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"1234.56"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0018(b: &mut Bencher) {
  let invocable_name = "varargs_001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#""foo bar""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}
