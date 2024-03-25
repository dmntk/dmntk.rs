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

from_examples!(DMN_3_0007);

const INPUT_DATA: &str = r#"{Day: 22, Hours: 12, Minutes: 59, Month: 11, Seconds: 1.3, Timezone: @"-PT1H", Year: 1999, dateString: "2015-12-24", dateTimeString: "2016-12-24T23:59:00-08:00", durationString: "P13DT2H14S", oneHour: PT1H, timeString: "00:00:01-01:00"}"#;

#[test]
fn _0001() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Date-Time", &ctx, r#"2016-12-24T23:59:00-08:00"#);
}

#[test]
fn _0002() {
  let ctx = context(INPUT_DATA);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "Date",
    &ctx,
    r#"{fromDateTime: 2016-12-24, fromString: 2015-12-24, fromYearMonthDay: 1999-11-22}"#,
  );
}

#[test]
fn _0003() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Time", &ctx, r#"00:00:01-01:00"#);
}

#[test]
fn _0004() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Date-Time2", &ctx, r#"2015-12-24T00:00:01-01:00"#);
}

#[test]
fn _0005() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Time2", &ctx, r#"00:00:01-01:00"#);
}

#[test]
fn _0006() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Time3", &ctx, r#"12:59:01.3-01:00"#);
}

#[test]
fn _0007() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dtDuration1", &ctx, r#"P13DT2H14S"#);
}

#[test]
fn _0008() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dtDuration2", &ctx, r#"P367DT6H58M59S"#);
}

#[test]
fn _0009() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "sumDurations", &ctx, r#"P380DT8H59M13S"#);
}

#[test]
fn _0010() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ymDuration2", &ctx, r#"P1Y"#);
}

#[test]
fn _0011() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "cDay", &ctx, r#"24"#);
}

#[test]
fn _0012() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "cYear", &ctx, r#"2015"#);
}

#[test]
fn _0013() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "cMonth", &ctx, r#"12"#);
}

#[test]
fn _0014() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "cHour", &ctx, r#"0"#);
}

#[test]
fn _0015() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "cMinute", &ctx, r#"0"#);
}

#[test]
fn _0016() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "cSecond", &ctx, r#"1"#);
}

#[test]
fn _0017() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "cOffset", &ctx, r#"-PT1H"#);
}

#[test]
fn _0018() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "years", &ctx, r#"1"#);
}

#[test]
fn _0019() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "seconds", &ctx, r#"14"#);
}
