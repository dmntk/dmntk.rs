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

from_examples!(DMN_3_0013);

#[test]
fn _0001() {
  let ctx = context(
    r#"{listA: [3, 1, 5, 4], stringList: ["a", "8", "Aa", "A", "10", "9"], tableB: [{col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}, {col1: 1, col2: 0, col3: 1, col4: 1}]}"#,
  );
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "sort1", &ctx, r#"[5, 4, 3, 1]"#);
}

#[test]
fn _0002() {
  let ctx = context(
    r#"{listA: [3, 1, 5, 4], stringList: ["a", "8", "Aa", "A", "10", "9"], tableB: [{col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}, {col1: 1, col2: 0, col3: 1, col4: 1}]}"#,
  );
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "sort2",
    &ctx,
    r#"[{col1: 1, col2: 0, col3: 1, col4: 1}, {col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}]"#,
  );
}

#[test]
fn _0003() {
  let ctx = context(
    r#"{listA: [3, 1, 5, 4], stringList: ["a", "8", "Aa", "A", "10", "9"], tableB: [{col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}, {col1: 1, col2: 0, col3: 1, col4: 1}]}"#,
  );
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "sort3", &ctx, r#"["10", "8", "9", "A", "Aa", "a"]"#);
}
