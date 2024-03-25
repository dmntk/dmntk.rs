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

from_examples!(DMN_3_1130);

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "during",
    &ctx,
    r#"{duringPRF: false, duringPRF2: false, duringPRF3: false, duringPRT: true, duringPRT2: true, duringPRT3: true, duringRRT: true, duringRRT2: true, duringRRT3: true, duringRRT4: true, duringRRT5: true, duringRRT6: true, duringRRT7: true, duringRRT8: true}"#,
  );
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "after",
    &ctx,
    r#"{afterPPF: false, afterPPT: true, afterPRF: false, afterPRT: true, afterPRT2: true, afterRPF: false, afterRPF2: false, afterRPT: true, afterRPT2: true, afterRRF: false, afterRRT: true, afterRRT2: true, afterRRT3: true}"#,
  );
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "started by",
    &ctx,
    r#"{startedbyRPF: false, startedbyRPF2: false, startedbyRPT: true, startedbyRRF: false, startedbyRRF2: false, startedbyRRT: true, startedbyRRT2: true, startedbyRRT3: true, startedbyRRT4: true, startedbyRRT5: true}"#,
  );
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "includes",
    &ctx,
    r#"{includesRPF: false, includesRPF2: false, includesRPF3: false, includesRPT: true, includesRPT2: true, includesRPT3: true, includesRRT: true, includesRRT2: true, includesRRT3: true, includesRRT4: true, includesRRT5: true, includesRRT6: true, includesRRT7: true, includesRRT8: true}"#,
  );
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "met by",
    &ctx,
    r#"{metbyRRF: false, metbyRRF2: false, metbyRRF3: false, metbyRRT: true}"#,
  );
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "before",
    &ctx,
    r#"{beforPPF: false, beforePPT: true, beforePRF: false, beforePRT: true, beforePRT2: true, beforeRPF: false, beforeRPT: true, beforeRPT2: true, beforeRRF: false, beforeRRT: true, beforeRRT2: true, beforeRRT3: true}"#,
  );
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "overlaps",
    &ctx,
    r#"{overlapsRRF: false, overlapsRRF2: false, overlapsRRF3: false, overlapsRRF4: false, overlapsRRF5: false, overlapsRRF6: false, overlapsRRF7: false, overlapsRRF8: false, overlapsRRT: true, overlapsRRT2: true, overlapsRRT3: true, overlapsRRT4: true, overlapsRRT5: true, overlapsRRT6: true}"#,
  );
}
