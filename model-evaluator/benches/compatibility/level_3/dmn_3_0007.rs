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
static CTX: Lazy<FeelContext> = Lazy::new(|| {
  context(
    r#"{
         Day: 22,
         Hours: 12,
         Minutes: 59,
         Month: 11,
         Seconds: 1.3,
         Timezone: @"-PT1H",
         Year: 1999,
         dateString: "2015-12-24",
         dateTimeString: "2016-12-24T23:59:00-08:00",
         durationString: "P13DT2H14S",
         oneHour: PT1H,
         timeString: "00:00:01-01:00"
    }"#,
  )
});

#[bench]
fn _0001(b: &mut Bencher) {
  let invocable_name = "Date-Time";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"2016-12-24T23:59:00-08:00"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let invocable_name = "Date";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &CTX,
    r#"{fromDateTime: 2016-12-24, fromString: 2015-12-24, fromYearMonthDay: 1999-11-22}"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let invocable_name = "Time";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"00:00:01-01:00"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let invocable_name = "Date-Time2";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"2015-12-24T00:00:01-01:00"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let invocable_name = "Time2";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"00:00:01-01:00"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let invocable_name = "Time3";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"12:59:01.3-01:00"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0007(b: &mut Bencher) {
  let invocable_name = "dtDuration1";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"P13DT2H14S"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0008(b: &mut Bencher) {
  let invocable_name = "dtDuration2";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"P367DT6H58M59S"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0009(b: &mut Bencher) {
  let invocable_name = "sumDurations";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"P380DT8H59M13S"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0010(b: &mut Bencher) {
  let invocable_name = "ymDuration2";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"P1Y"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0011(b: &mut Bencher) {
  let invocable_name = "cDay";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"24"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0012(b: &mut Bencher) {
  let invocable_name = "cYear";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"2015"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0013(b: &mut Bencher) {
  let invocable_name = "cMonth";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"12"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0014(b: &mut Bencher) {
  let invocable_name = "cHour";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"0"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0015(b: &mut Bencher) {
  let invocable_name = "cMinute";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"0"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0016(b: &mut Bencher) {
  let invocable_name = "cSecond";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0017(b: &mut Bencher) {
  let invocable_name = "cOffset";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"-PT1H"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0018(b: &mut Bencher) {
  let invocable_name = "years";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}

#[bench]
fn _0019(b: &mut Bencher) {
  let invocable_name = "seconds";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &CTX, r#"14"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &CTX));
}
