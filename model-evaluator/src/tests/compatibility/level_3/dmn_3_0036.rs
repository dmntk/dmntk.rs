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

from_examples!(DMN_3_0036);

static_context!(
  CTX1,
  r#"{"Another Date": @"2018-07-31", "Another Date and Time": @"2018-07-31T17:13:00Z", "Another Days and Time Duration": @"PT12H", "Another String": "Hello", "Another Time": @"17:13:00", "Another Years and Months Duration": @"P8M", "Another boolean": false, "Another number": 15, Complex: { aBoolean: true, aDate: @"2018-07-30", aDateTime: @"2018-07-30T16:12:00Z", aDaysAndTimeDuration: @"PT10H", aNumber: 10, aString: "Hi", aTime: @"16:11:00", aYearsAndMonthsDuration: @"P5M"}}"#
);

static_context!(
  CTX2,
  r#"{"Another Date": @"2018-07-29", "Another Date and Time": @"2018-07-29T15:13:00Z", "Another Days and Time Duration": @"PT8H",  "Another String": "Hello", "Another Time": @"15:13:00", "Another Years and Months Duration": @"P3M", "Another boolean": false, "Another number": 5,  Complex: { aBoolean: true, aDate: @"2018-07-30", aDateTime: @"2018-07-30T16:12:00Z", aDaysAndTimeDuration: @"PT10H", aNumber: 10, aString: "Hi", aTime: @"16:11:00", aYearsAndMonthsDuration: @"P5M"}}"#
);

static_context!(
  CTX3,
  r#"{"Another Date": @"2018-07-30", "Another Date and Time": @"2018-07-30T16:12:00Z", "Another Days and Time Duration": @"PT10H", "Another String": "Hi",    "Another Time": @"16:11:00", "Another Years and Months Duration": @"P5M", "Another boolean": true,  "Another number": 10, Complex: { aBoolean: true, aDate: @"2018-07-30", aDateTime: @"2018-07-30T16:12:00Z", aDaysAndTimeDuration: @"PT10H", aNumber: 10, aString: "Hi", aTime: @"16:11:00", aYearsAndMonthsDuration: @"P5M"}}"#
);

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare String", &CTX1, r#""Different String""#);
}

#[test]
fn _0002() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Date", &CTX1, r#""Future Date""#);
}

#[test]
fn _0003() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Number", &CTX1, r#""Bigger""#);
}

#[test]
fn _0004() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Date and Time", &CTX1, r#""Future date time""#);
}

#[test]
fn _0005() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Days and Time Duration", &CTX1, r#""Longer duration""#);
}

#[test]
fn _0006() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Years and Months Duration", &CTX1, r#""Longer duration""#);
}

#[test]
fn _0007() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Time", &CTX1, r#""Future Time""#);
}

#[test]
fn _0008() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Boolean", &CTX1, r#""Not same boolean""#);
}

#[test]
fn _0009() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare String", &CTX2, r#""Different String""#);
}

#[test]
fn _0010() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Date", &CTX2, r#""Past Date""#);
}

#[test]
fn _0011() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Number", &CTX2, r#""Smaller""#);
}

#[test]
fn _0012() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Date and Time", &CTX2, r#""Past date time""#);
}

#[test]
fn _0013() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Days and Time Duration", &CTX2, r#""Shorter duration""#);
}

#[test]
fn _0014() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Years and Months Duration", &CTX2, r#""Shorter duration""#);
}

#[test]
fn _0015() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Time", &CTX2, r#""Past Time""#);
}

#[test]
fn _0016() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Boolean", &CTX2, r#""Not same boolean""#);
}

#[test]
fn _0017() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare String", &CTX3, r#""Same String""#);
}

#[test]
fn _0018() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Date", &CTX3, r#""Same Date""#);
}

#[test]
fn _0019() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Number", &CTX3, r#""Equals""#);
}

#[test]
fn _0020() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Date and Time", &CTX3, r#""Same date time""#);
}

#[test]
fn _0021() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Days and Time Duration", &CTX3, r#""Same duration""#);
}

#[test]
fn _0022() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Years and Months Duration", &CTX3, r#""Same duration""#);
}

#[test]
fn _0023() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Time", &CTX3, r#""Same Time""#);
}

#[test]
fn _0024() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Compare Boolean", &CTX3, r#""Same boolean""#);
}
