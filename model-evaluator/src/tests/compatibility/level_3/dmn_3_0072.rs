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

from_examples!(DMN_3_0072);

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_001", &ctx, r#"true"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_001_a", &ctx, r#"false"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_002", &ctx, r#"true"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_002_a", &ctx, r#"false"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_003", &ctx, r#"true"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_003_1", &ctx, r#"true"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_003_a", &ctx, r#"false"#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_004", &ctx, r#"true"#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_004_a", &ctx, r#"false"#);
}

#[test]
fn _0010() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_005", &ctx, r#"true"#);
}

#[test]
fn _0011() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_005_1", &ctx, r#"true"#);
}

#[test]
fn _0012() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_005_a", &ctx, r#"false"#);
}

#[test]
fn _0013() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_006", &ctx, r#"true"#);
}

#[test]
fn _0014() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_006_a", &ctx, r#"false"#);
}

#[test]
fn _0015() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_007_a", &ctx, r#"false"#);
}

#[test]
fn _0016() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_007_b", &ctx, r#"false"#);
}

#[test]
fn _0017() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_007_c", &ctx, r#"true"#);
}

#[test]
fn _0018() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_007_d", &ctx, r#"false"#);
}

#[test]
fn _0019() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_007_e", &ctx, r#"false"#);
}

#[test]
fn _0020() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_008_a", &ctx, r#"false"#);
}

#[test]
fn _0021() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_008_b", &ctx, r#"false"#);
}

#[test]
fn _0022() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_008_c", &ctx, r#"true"#);
}

#[test]
fn _0023() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_008_d", &ctx, r#"true"#);
}

#[test]
fn _0024() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_008_e", &ctx, r#"false"#);
}

#[test]
fn _0025() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_009_a", &ctx, r#"false"#);
}

#[test]
fn _0026() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_009_b", &ctx, r#"true"#);
}

#[test]
fn _0027() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_009_c", &ctx, r#"true"#);
}

#[test]
fn _0028() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_009_d", &ctx, r#"false"#);
}

#[test]
fn _0029() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_009_e", &ctx, r#"false"#);
}

#[test]
fn _0030() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_010_a", &ctx, r#"false"#);
}

#[test]
fn _0031() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_010_b", &ctx, r#"true"#);
}

#[test]
fn _0032() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_010_c", &ctx, r#"true"#);
}

#[test]
fn _0033() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_010_d", &ctx, r#"true"#);
}

#[test]
fn _0034() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_010_e", &ctx, r#"false"#);
}

#[test]
fn _0035() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_011", &ctx, r#"true"#);
}

#[test]
fn _0036() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_014", &ctx, r#"true"#);
}

#[test]
fn _0037() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "number_014_a", &ctx, r#"false"#);
}

#[test]
fn _0038() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_001", &ctx, r#"true"#);
}

#[test]
fn _0039() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_001_a", &ctx, r#"false"#);
}

#[test]
fn _0040() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_002", &ctx, r#"true"#);
}

#[test]
fn _0041() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_002_a", &ctx, r#"false"#);
}

#[test]
fn _0042() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_003", &ctx, r#"true"#);
}

#[test]
fn _0043() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_003_1", &ctx, r#"true"#);
}

#[test]
fn _0044() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_003_a", &ctx, r#"false"#);
}

#[test]
fn _0045() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_004", &ctx, r#"true"#);
}

#[test]
fn _0046() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_004_a", &ctx, r#"false"#);
}

#[test]
fn _0047() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_005", &ctx, r#"true"#);
}

#[test]
fn _0048() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_005_1", &ctx, r#"true"#);
}

#[test]
fn _0049() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_005_a", &ctx, r#"false"#);
}

#[test]
fn _0050() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_006", &ctx, r#"true"#);
}

#[test]
fn _0051() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_006_a", &ctx, r#"false"#);
}

#[test]
fn _0052() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_007_a", &ctx, r#"false"#);
}

#[test]
fn _0053() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_007_b", &ctx, r#"false"#);
}

#[test]
fn _0054() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_007_c", &ctx, r#"true"#);
}

#[test]
fn _0055() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_007_d", &ctx, r#"false"#);
}

#[test]
fn _0056() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_007_e", &ctx, r#"false"#);
}

#[test]
fn _0057() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_008_a", &ctx, r#"false"#);
}

#[test]
fn _0058() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_008_b", &ctx, r#"false"#);
}

#[test]
fn _0059() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_008_c", &ctx, r#"true"#);
}

#[test]
fn _0060() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_008_d", &ctx, r#"true"#);
}

#[test]
fn _0061() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_008_e", &ctx, r#"false"#);
}

#[test]
fn _0062() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_009_a", &ctx, r#"false"#);
}

#[test]
fn _0063() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_009_b", &ctx, r#"true"#);
}

#[test]
fn _0064() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_009_c", &ctx, r#"true"#);
}

#[test]
fn _0065() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_009_d", &ctx, r#"false"#);
}

#[test]
fn _0066() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_009_e", &ctx, r#"false"#);
}

#[test]
fn _0067() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_010_a", &ctx, r#"false"#);
}

#[test]
fn _0068() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_010_b", &ctx, r#"true"#);
}

#[test]
fn _0069() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_010_c", &ctx, r#"true"#);
}

#[test]
fn _0070() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_010_d", &ctx, r#"true"#);
}

#[test]
fn _0071() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_010_e", &ctx, r#"false"#);
}

#[test]
fn _0072() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_011", &ctx, r#"true"#);
}

#[test]
fn _0073() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_014", &ctx, r#"true"#);
}

#[test]
fn _0074() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "string_014_a", &ctx, r#"false"#);
}

#[test]
fn _0075() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "boolean_001", &ctx, r#"true"#);
}

#[test]
fn _0076() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "boolean_001_a", &ctx, r#"false"#);
}

#[test]
fn _0077() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "boolean_011", &ctx, r#"true"#);
}

#[test]
fn _0078() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "boolean_014", &ctx, r#"true"#);
}

#[test]
fn _0079() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "boolean_014_a", &ctx, r#"false"#);
}

#[test]
fn _0080() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_001", &ctx, r#"true"#);
}

#[test]
fn _0081() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_001_a", &ctx, r#"false"#);
}

#[test]
fn _0082() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_002", &ctx, r#"true"#);
}

#[test]
fn _0083() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_002_a", &ctx, r#"false"#);
}

#[test]
fn _0084() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_003", &ctx, r#"true"#);
}

#[test]
fn _0085() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_003_1", &ctx, r#"true"#);
}

#[test]
fn _0086() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_003_a", &ctx, r#"false"#);
}

#[test]
fn _0087() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_004", &ctx, r#"true"#);
}

#[test]
fn _0088() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_004_a", &ctx, r#"false"#);
}

#[test]
fn _0089() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_005", &ctx, r#"true"#);
}

#[test]
fn _0090() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_005_1", &ctx, r#"true"#);
}

#[test]
fn _0091() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_005_a", &ctx, r#"false"#);
}

#[test]
fn _0092() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_006", &ctx, r#"true"#);
}

#[test]
fn _0093() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_006_a", &ctx, r#"false"#);
}

#[test]
fn _0094() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_007_a", &ctx, r#"false"#);
}

#[test]
fn _0095() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_007_b", &ctx, r#"false"#);
}

#[test]
fn _0096() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_007_c", &ctx, r#"true"#);
}

#[test]
fn _0097() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_007_d", &ctx, r#"false"#);
}

#[test]
fn _0098() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_007_e", &ctx, r#"false"#);
}

#[test]
fn _0099() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_008_a", &ctx, r#"false"#);
}

#[test]
fn _0100() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_008_b", &ctx, r#"false"#);
}

#[test]
fn _0101() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_008_c", &ctx, r#"true"#);
}

#[test]
fn _0102() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_008_d", &ctx, r#"true"#);
}

#[test]
fn _0103() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_008_e", &ctx, r#"false"#);
}

#[test]
fn _0104() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_009_a", &ctx, r#"false"#);
}

#[test]
fn _0105() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_009_b", &ctx, r#"true"#);
}

#[test]
fn _0106() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_009_c", &ctx, r#"true"#);
}

#[test]
fn _0107() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_009_d", &ctx, r#"false"#);
}

#[test]
fn _0108() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_009_e", &ctx, r#"false"#);
}

#[test]
fn _0109() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_010_a", &ctx, r#"false"#);
}

#[test]
fn _0110() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_010_b", &ctx, r#"true"#);
}

#[test]
fn _0111() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_010_c", &ctx, r#"true"#);
}

#[test]
fn _0112() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_010_d", &ctx, r#"true"#);
}

#[test]
fn _0113() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_010_e", &ctx, r#"false"#);
}

#[test]
fn _0114() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_011", &ctx, r#"true"#);
}

#[test]
fn _0115() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_014", &ctx, r#"true"#);
}

#[test]
fn _0116() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_014_a", &ctx, r#"false"#);
}

#[test]
fn _0117() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_001", &ctx, r#"true"#);
}

#[test]
fn _0118() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_001_a", &ctx, r#"false"#);
}

#[test]
fn _0119() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_002", &ctx, r#"true"#);
}

#[test]
fn _0120() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_002_a", &ctx, r#"false"#);
}

#[test]
fn _0121() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_003", &ctx, r#"true"#);
}

#[test]
fn _0122() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_003_1", &ctx, r#"true"#);
}

#[test]
fn _0123() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_003_a", &ctx, r#"false"#);
}

#[test]
fn _0124() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_004", &ctx, r#"true"#);
}

#[test]
fn _0125() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_004_a", &ctx, r#"false"#);
}

#[test]
fn _0126() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_005", &ctx, r#"true"#);
}

#[test]
fn _0127() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_005_1", &ctx, r#"true"#);
}

#[test]
fn _0128() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_005_a", &ctx, r#"false"#);
}

#[test]
fn _0129() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_006", &ctx, r#"true"#);
}

#[test]
fn _0130() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_006_a", &ctx, r#"false"#);
}

#[test]
fn _0131() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_007_a", &ctx, r#"false"#);
}

#[test]
fn _0132() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_007_b", &ctx, r#"false"#);
}

#[test]
fn _0133() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_007_c", &ctx, r#"true"#);
}

#[test]
fn _0134() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_007_d", &ctx, r#"false"#);
}

#[test]
fn _0135() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_007_e", &ctx, r#"false"#);
}

#[test]
fn _0136() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_008_a", &ctx, r#"false"#);
}

#[test]
fn _0137() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_008_b", &ctx, r#"false"#);
}

#[test]
fn _0138() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_008_c", &ctx, r#"true"#);
}

#[test]
fn _0139() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_008_d", &ctx, r#"true"#);
}

#[test]
fn _0140() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_008_e", &ctx, r#"false"#);
}

#[test]
fn _0141() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_009_a", &ctx, r#"false"#);
}

#[test]
fn _0142() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_009_b", &ctx, r#"true"#);
}

#[test]
fn _0143() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_009_c", &ctx, r#"true"#);
}

#[test]
fn _0144() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_009_d", &ctx, r#"false"#);
}

#[test]
fn _0145() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_009_e", &ctx, r#"false"#);
}

#[test]
fn _0146() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_010_a", &ctx, r#"false"#);
}

#[test]
fn _0147() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_010_b", &ctx, r#"true"#);
}

#[test]
fn _0148() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_010_c", &ctx, r#"true"#);
}

#[test]
fn _0149() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_010_d", &ctx, r#"true"#);
}

#[test]
fn _0150() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_010_e", &ctx, r#"false"#);
}

#[test]
fn _0151() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_011", &ctx, r#"true"#);
}

#[test]
fn _0152() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_014", &ctx, r#"true"#);
}

#[test]
fn _0153() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_014_a", &ctx, r#"false"#);
}

#[test]
fn _0154() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_001", &ctx, r#"true"#);
}

#[test]
fn _0155() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_001_a", &ctx, r#"false"#);
}

#[test]
fn _0156() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_002", &ctx, r#"true"#);
}

#[test]
fn _0157() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_002_a", &ctx, r#"false"#);
}

#[test]
fn _0158() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_003", &ctx, r#"true"#);
}

#[test]
fn _0159() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_003_1", &ctx, r#"true"#);
}

#[test]
fn _0160() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_003_a", &ctx, r#"false"#);
}

#[test]
fn _0161() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_004", &ctx, r#"true"#);
}

#[test]
fn _0162() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_004_a", &ctx, r#"false"#);
}

#[test]
fn _0163() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_005", &ctx, r#"true"#);
}

#[test]
fn _0164() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_005_1", &ctx, r#"true"#);
}

#[test]
fn _0165() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_005_a", &ctx, r#"false"#);
}

#[test]
fn _0166() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_006", &ctx, r#"true"#);
}

#[test]
fn _0167() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_006_a", &ctx, r#"false"#);
}

#[test]
fn _0168() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_007_a", &ctx, r#"false"#);
}

#[test]
fn _0169() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_007_b", &ctx, r#"false"#);
}

#[test]
fn _0170() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_007_c", &ctx, r#"true"#);
}

#[test]
fn _0171() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_007_d", &ctx, r#"false"#);
}

#[test]
fn _0172() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_007_e", &ctx, r#"false"#);
}

#[test]
fn _0173() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_008_a", &ctx, r#"false"#);
}

#[test]
fn _0174() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_008_b", &ctx, r#"false"#);
}

#[test]
fn _0175() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_008_c", &ctx, r#"true"#);
}

#[test]
fn _0176() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_008_d", &ctx, r#"true"#);
}

#[test]
fn _0177() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_008_e", &ctx, r#"false"#);
}

#[test]
fn _0178() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_009_a", &ctx, r#"false"#);
}

#[test]
fn _0179() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_009_b", &ctx, r#"true"#);
}

#[test]
fn _0180() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_009_c", &ctx, r#"true"#);
}

#[test]
fn _0181() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_009_d", &ctx, r#"false"#);
}

#[test]
fn _0182() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_009_e", &ctx, r#"false"#);
}

#[test]
fn _0183() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_010_a", &ctx, r#"false"#);
}

#[test]
fn _0184() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_010_b", &ctx, r#"true"#);
}

#[test]
fn _0185() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_010_c", &ctx, r#"true"#);
}

#[test]
fn _0186() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_010_d", &ctx, r#"true"#);
}

#[test]
fn _0187() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_010_e", &ctx, r#"false"#);
}

#[test]
fn _0188() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_011", &ctx, r#"true"#);
}

#[test]
fn _0189() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_014", &ctx, r#"true"#);
}

#[test]
fn _0190() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dateTime_014_a", &ctx, r#"false"#);
}

#[test]
fn _0191() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "list_001", &ctx, r#"true"#);
}

#[test]
fn _0192() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "list_001_a", &ctx, r#"false"#);
}

#[test]
fn _0193() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "list_011_a", &ctx, r#"true"#);
}

#[test]
fn _0194() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "list_011_b", &ctx, r#"false"#);
}

#[test]
fn _0195() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "list_014_a", &ctx, r#"false"#);
}

#[test]
fn _0196() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "context_001", &ctx, r#"true"#);
}

#[test]
fn _0197() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "context_001_a", &ctx, r#"false"#);
}

#[test]
fn _0198() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "context_011", &ctx, r#"true"#);
}

#[test]
fn _0199() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "context_014", &ctx, r#"true"#);
}

#[test]
fn _0200() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "context_014_a", &ctx, r#"false"#);
}

#[test]
fn _0201() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_001", &ctx, r#"true"#);
}

#[test]
fn _0202() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_001_a", &ctx, r#"false"#);
}

#[test]
fn _0203() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_002", &ctx, r#"true"#);
}

#[test]
fn _0204() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_002_a", &ctx, r#"false"#);
}

#[test]
fn _0205() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_003", &ctx, r#"true"#);
}

#[test]
fn _0206() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_003_1", &ctx, r#"true"#);
}

#[test]
fn _0207() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_003_a", &ctx, r#"false"#);
}

#[test]
fn _0208() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_004", &ctx, r#"true"#);
}

#[test]
fn _0209() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_004_a", &ctx, r#"false"#);
}

#[test]
fn _0210() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_005", &ctx, r#"true"#);
}

#[test]
fn _0211() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_005_1", &ctx, r#"true"#);
}

#[test]
fn _0212() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_005_a", &ctx, r#"false"#);
}

#[test]
fn _0213() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_006", &ctx, r#"true"#);
}

#[test]
fn _0214() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_006_a", &ctx, r#"false"#);
}

#[test]
fn _0215() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_007_a", &ctx, r#"false"#);
}

#[test]
fn _0216() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_007_b", &ctx, r#"false"#);
}

#[test]
fn _0217() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_007_c", &ctx, r#"true"#);
}

#[test]
fn _0218() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_007_d", &ctx, r#"false"#);
}

#[test]
fn _0219() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_007_e", &ctx, r#"false"#);
}

#[test]
fn _0220() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_008_a", &ctx, r#"false"#);
}

#[test]
fn _0221() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_008_b", &ctx, r#"false"#);
}

#[test]
fn _0222() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_008_c", &ctx, r#"true"#);
}

#[test]
fn _0223() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_008_d", &ctx, r#"true"#);
}

#[test]
fn _0224() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_008_e", &ctx, r#"false"#);
}

#[test]
fn _0225() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_009_a", &ctx, r#"false"#);
}

#[test]
fn _0226() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_009_b", &ctx, r#"true"#);
}

#[test]
fn _0227() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_009_c", &ctx, r#"true"#);
}

#[test]
fn _0228() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_009_d", &ctx, r#"false"#);
}

#[test]
fn _0229() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_009_e", &ctx, r#"false"#);
}

#[test]
fn _0230() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_010_a", &ctx, r#"false"#);
}

#[test]
fn _0231() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_010_b", &ctx, r#"true"#);
}

#[test]
fn _0232() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_010_c", &ctx, r#"true"#);
}

#[test]
fn _0233() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_010_d", &ctx, r#"true"#);
}

#[test]
fn _0234() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_010_e", &ctx, r#"false"#);
}

#[test]
fn _0235() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_011", &ctx, r#"true"#);
}

#[test]
fn _0236() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_014", &ctx, r#"true"#);
}

#[test]
fn _0237() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "ym_duration_014_a", &ctx, r#"false"#);
}

#[test]
fn _0238() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_001", &ctx, r#"true"#);
}

#[test]
fn _0239() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_001_a", &ctx, r#"false"#);
}

#[test]
fn _0240() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_002", &ctx, r#"true"#);
}

#[test]
fn _0241() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_002_a", &ctx, r#"false"#);
}

#[test]
fn _0242() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_003", &ctx, r#"true"#);
}

#[test]
fn _0243() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_003_1", &ctx, r#"true"#);
}

#[test]
fn _0244() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_003_a", &ctx, r#"false"#);
}

#[test]
fn _0245() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_004", &ctx, r#"true"#);
}

#[test]
fn _0246() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_004_a", &ctx, r#"false"#);
}

#[test]
fn _0247() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_005", &ctx, r#"true"#);
}

#[test]
fn _0248() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_005_1", &ctx, r#"true"#);
}

#[test]
fn _0249() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_005_a", &ctx, r#"false"#);
}

#[test]
fn _0250() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_006", &ctx, r#"true"#);
}

#[test]
fn _0251() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_006_a", &ctx, r#"false"#);
}

#[test]
fn _0252() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_007_a", &ctx, r#"false"#);
}

#[test]
fn _0253() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_007_b", &ctx, r#"false"#);
}

#[test]
fn _0254() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_007_c", &ctx, r#"true"#);
}

#[test]
fn _0255() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_007_d", &ctx, r#"false"#);
}

#[test]
fn _0256() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_007_e", &ctx, r#"false"#);
}

#[test]
fn _0257() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_008_a", &ctx, r#"false"#);
}

#[test]
fn _0258() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_008_b", &ctx, r#"false"#);
}

#[test]
fn _0259() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_008_c", &ctx, r#"true"#);
}

#[test]
fn _0260() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_008_d", &ctx, r#"true"#);
}

#[test]
fn _0261() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_008_e", &ctx, r#"false"#);
}

#[test]
fn _0262() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_009_a", &ctx, r#"false"#);
}

#[test]
fn _0263() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_009_b", &ctx, r#"true"#);
}

#[test]
fn _0264() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_009_c", &ctx, r#"true"#);
}

#[test]
fn _0265() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_009_d", &ctx, r#"false"#);
}

#[test]
fn _0266() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_009_e", &ctx, r#"false"#);
}

#[test]
fn _0267() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_010_a", &ctx, r#"false"#);
}

#[test]
fn _0268() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_010_b", &ctx, r#"true"#);
}

#[test]
fn _0269() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_010_c", &ctx, r#"true"#);
}

#[test]
fn _0270() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_010_d", &ctx, r#"true"#);
}

#[test]
fn _0271() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_010_e", &ctx, r#"false"#);
}

#[test]
fn _0272() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_011", &ctx, r#"true"#);
}

#[test]
fn _0273() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_014", &ctx, r#"true"#);
}

#[test]
fn _0274() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "dt_duration_014_a", &ctx, r#"false"#);
}
