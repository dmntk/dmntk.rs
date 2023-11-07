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

from_examples!(DMN_3_0087);
static CTX: Lazy<FeelContext> = Lazy::new(|| {
    context(
        r#"
    {
      Applicant data: {
        Age: 51,
        EmploymentStatus: "EMPLOYED",
        ExistingCustomer: false,
        MartitalStatus: "M",
        Monthly: {
          Expenses: 10000,
          Income: 100000,
          Repayments: 2500
        }
      },
      Bureau data: {
        Bankrupt: false,
        CreditScore: 600
      },
      Requested product: {
        Amount: 100000,
        ProductType: "STANDARD LOAN",
        Rate: 0.08,
        Term: 36
      }
    }"#,
    )
});

#[bench]
fn _0001(b: &mut Bencher) {
    let invocable_name = "Strategy";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        invocable_name,
        &CTX,
        r#""THROUGH""#,
    );
    b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0002(b: &mut Bencher) {
    let invocable_name = "Routing";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        invocable_name,
        &CTX,
        r#""ACCEPT""#,
    );
    b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0003(b: &mut Bencher) {
    let invocable_name = "Application risk score";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        invocable_name,
        &CTX,
        r#"138"#,
    );
    b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0004(b: &mut Bencher) {
    let invocable_name = "Pre-bureau risk category";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        invocable_name,
        &CTX,
        r#""VERY LOW""#,
    );
    b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0005(b: &mut Bencher) {
    let invocable_name = "Bureau call type";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        invocable_name,
        &CTX,
        r#""NONE""#,
    );
    b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0006(b: &mut Bencher) {
    let invocable_name = "Eligibility";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        invocable_name,
        &CTX,
        r#""ELIGIBLE""#,
    );
    b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0007(b: &mut Bencher) {
    let invocable_name = "Post-bureau affordability";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        invocable_name,
        &CTX,
        r#"true"#,
    );
    b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0008(b: &mut Bencher) {
    let invocable_name = "Strategy";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        invocable_name,
        &CTX,
        r#""THROUGH""#,
    );
    b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0009(b: &mut Bencher) {
    let invocable_name = "Routing";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        invocable_name,
        &CTX,
        r#""ACCEPT""#,
    );
    b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}
