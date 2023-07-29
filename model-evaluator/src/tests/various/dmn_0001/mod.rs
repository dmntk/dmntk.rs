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

//! ???

use super::super::*;

const DMN_0001: &str = include_str!("_0001.dmn");

model_evaluator!(DMN_0001);

const NAMESPACE: &str = "https://dmntk.io/selector";

#[test]
fn _0001() {
  let ctx = context(r#"{ Days: 6, Bounds: { Min: null, Max: 360 } }"#);
  assert_decision(&MODEL_EVALUATOR, NAMESPACE, "Selector", &ctx, r#""7a""#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{ Days: 7, Bounds: { Min: 7, Max: 360 } }"#);
  assert_decision(&MODEL_EVALUATOR, NAMESPACE, "Selector", &ctx, r#""7b""#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{ Days: 8, Bounds: { Min: 7, Max: 360 } }"#);
  assert_decision(&MODEL_EVALUATOR, NAMESPACE, "Selector", &ctx, r#""(7..360)""#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{ Days: 360, Bounds: { Min: 7, Max: 360 } }"#);
  assert_decision(&MODEL_EVALUATOR, NAMESPACE, "Selector", &ctx, r#""360+""#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{ Days: 365, Bounds: { Min: 7, Max: 360 } }"#);
  assert_decision(&MODEL_EVALUATOR, NAMESPACE, "Selector", &ctx, r#""360+""#);
}
