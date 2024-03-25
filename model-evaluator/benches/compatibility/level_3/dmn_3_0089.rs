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

static MODEL_EVALUATOR_A: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_A]));
static MODEL_EVALUATOR_B1: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_B1, DMN_3_0089_MODEL_A]));
static MODEL_EVALUATOR_B2: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_B2, DMN_3_0089_MODEL_A]));
static MODEL_EVALUATOR_C: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_C, DMN_3_0089_MODEL_B1, DMN_3_0089_MODEL_B2, DMN_3_0089_MODEL_A]));

const NAMESPACE_A: &str = "http://www.trisotech.com/definitions/_ae5b3c17-1ac3-4e1d-b4f9-2cf861aec6d9";
const NAMESPACE_B1: &str = "http://www.trisotech.com/definitions/_2a1d771a-a899-4fef-abd6-fc894332337c";
const NAMESPACE_B2: &str = "http://www.trisotech.com/definitions/_9d46ece4-a96c-4cb0-abc0-0ca121ac3768";
const NAMESPACE_C: &str = "http://www.trisotech.com/definitions/_10435dcd-8774-4575-a338-49dd554a0928";

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#" { Person name: "Jenny" } "#);
  let invocable_name = "Greet the Person";
  assert_decision(&MODEL_EVALUATOR_A, NAMESPACE_A, invocable_name, &ctx, r#""Hello, Jenny""#);
  b.iter(|| MODEL_EVALUATOR_A.evaluate_invocable(NAMESPACE_A, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#" { Model A: { Person name: "John" }} "#);
  let invocable_name = "Evaluating Say Hello";
  assert_decision(&MODEL_EVALUATOR_B1, NAMESPACE_B1, invocable_name, &ctx, r#""Evaluating Say Hello to: Hello, John""#);
  b.iter(|| MODEL_EVALUATOR_B1.evaluate_invocable(NAMESPACE_B1, invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#" { Model A: { Person name: "Peter" }} "#);
  let invocable_name = "Evaluating B2 Say Hello";
  assert_decision(&MODEL_EVALUATOR_B2, NAMESPACE_B2, invocable_name, &ctx, r#""Evaluating Say Hello to: Hello, Peter""#);
  b.iter(|| MODEL_EVALUATOR_B2.evaluate_invocable(NAMESPACE_B2, invocable_name, &ctx));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(
    r#"{ 
      Model B1: { Model A: { Person name: "Bob" }},
      Model B2: { Model A: { Person name: "John" }} 
    }"#,
  );
  let invocable_name = "Model C Decision based on Bs";
  assert_decision(
    &MODEL_EVALUATOR_C,
    NAMESPACE_C,
    invocable_name,
    &ctx,
    r#""B1: Evaluating Say Hello to: Hello, Bob; B2: Evaluating Say Hello to: Hello, John""#,
  );
  b.iter(|| MODEL_EVALUATOR_C.evaluate_invocable(NAMESPACE_C, invocable_name, &ctx));
}
