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

from_examples!(DMN_3_0004);

const APPLICANT_DATA: &str = r#"
  {
    ApplicantData:  {
      Age:  35,
      EmploymentStatus:  "EMPLOYED",
      ExistingCustomer:  true,
      MaritalStatus:  "M",
      Monthly:  {
        Expenses:  2000,
        Income:  6000,
        Repayments:  0
      }
    },
    BureauData:  {
      Bankrupt:  false,
      CreditScore:  649
    },
    RequestedProduct:  {
      Amount:  350000,
      ProductType:  "STANDARD LOAN",
      Rate:  0.0395,
      Term:  360
    },
    SupportingDocuments:  "YES"
  }
"#;

#[test]
fn _0001() {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Adjudication", &ctx, r#""ACCEPT""#);
}

#[test]
fn _0002() {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ApplicationRiskScore", &ctx, r#"130"#);
}

#[test]
fn _0003() {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Pre-bureauRiskCategory", &ctx, r#""LOW""#);
}

#[test]
fn _0004() {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "BureauCallType", &ctx, r#""MINI""#);
}

#[test]
fn _0005() {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Post-bureauRiskCategory", &ctx, r#""LOW""#);
}

#[test]
fn _0006() {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "RequiredMonthlyInstallment", &ctx, r#"1680.8803256086347968"#);
}

#[test]
fn _0007() {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Pre-bureauAffordability", &ctx, r#"true"#);
}

#[test]
fn _0008() {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Eligibility", &ctx, r#""ELIGIBLE""#);
}

#[test]
fn _0009() {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Strategy", &ctx, r#""BUREAU""#);
}

#[test]
fn _0010() {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Post-bureauAffordability", &ctx, r#"true"#);
}

#[test]
fn _0011() {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Routing", &ctx, r#""ACCEPT""#);
}
