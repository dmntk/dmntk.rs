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

from_examples!(DMN_3_0074);

#[test]
fn _0001() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "context_001",
        &ctx,
        r#""foo""#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "date_001",
        &ctx,
        r#"2018"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "date_002",
        &ctx,
        r#"12"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "date_003",
        &ctx,
        r#"10"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(r#"{}"#);
    assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_004", &ctx, r#"1"#);
}

#[test]
fn _0006() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dateTime_001",
        &ctx,
        r#"2018"#,
    );
}

#[test]
fn _0007() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dateTime_002",
        &ctx,
        r#"12"#,
    );
}

#[test]
fn _0008() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dateTime_003",
        &ctx,
        r#"10"#,
    );
}

#[test]
fn _0009() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dateTime_004",
        &ctx,
        r#"1"#,
    );
}

#[test]
fn _0010() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dateTime_005",
        &ctx,
        r#"10"#,
    );
}

#[test]
fn _0011() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dateTime_005_a",
        &ctx,
        r#"0"#,
    );
}

#[test]
fn _0012() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dateTime_006",
        &ctx,
        r#"30"#,
    );
}

#[test]
fn _0013() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dateTime_006_a",
        &ctx,
        r#"0"#,
    );
}

#[test]
fn _0014() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dateTime_007",
        &ctx,
        r#"1"#,
    );
}

#[test]
fn _0015() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dateTime_007_a",
        &ctx,
        r#"0"#,
    );
}

#[test]
fn _0016() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dateTime_008",
        &ctx,
        r#"PT5H"#,
    );
}

#[test]
fn _0017() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dateTime_008_a",
        &ctx,
        r#"null(could not retrieve time offset for date and time)"#,
    );
}

#[test]
fn _0018() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dateTime_009",
        &ctx,
        r#""Etc/UTC""#,
    );
}

#[test]
fn _0019() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dateTime_009_a",
        &ctx,
        r#"null(could not retrieve timezone for date and time)"#,
    );
}

#[test]
fn _0020() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "time_001",
        &ctx,
        r#"10"#,
    );
}

#[test]
fn _0021() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "time_002",
        &ctx,
        r#"30"#,
    );
}

#[test]
fn _0022() {
    let ctx = context(r#"{}"#);
    assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "time_003", &ctx, r#"1"#);
}

#[test]
fn _0023() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "time_004",
        &ctx,
        r#"PT5H"#,
    );
}

#[test]
fn _0024() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "time_004_a",
        &ctx,
        r#"null(could not retrieve time offset for time)"#,
    );
}

#[test]
fn _0025() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "time_005_a",
        &ctx,
        r#"null(could not retrieve timezone for time)"#,
    );
}

#[test]
fn _0026() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "ym_duration_001",
        &ctx,
        r#"1"#,
    );
}

#[test]
fn _0027() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "ym_duration_001_a",
        &ctx,
        r#"0"#,
    );
}

#[test]
fn _0028() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "ym_duration_002",
        &ctx,
        r#"2"#,
    );
}

#[test]
fn _0029() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "ym_duration_002_a",
        &ctx,
        r#"0"#,
    );
}

#[test]
fn _0030() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "ym_duration_003",
        &ctx,
        r#"null(no such property in years and months duration: days)"#,
    );
}

#[test]
fn _0031() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "ym_duration_004",
        &ctx,
        r#"null(no such property in years and months duration: hours)"#,
    );
}

#[test]
fn _0032() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "ym_duration_005",
        &ctx,
        r#"null(no such property in years and months duration: minutes)"#,
    );
}

#[test]
fn _0033() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "ym_duration_006",
        &ctx,
        r#"null(no such property in years and months duration: seconds)"#,
    );
}

#[test]
fn _0034() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dt_duration_001",
        &ctx,
        r#"null(no such property in days and time duration: years)"#,
    );
}

#[test]
fn _0035() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dt_duration_002",
        &ctx,
        r#"null(no such property in days and time duration: months)"#,
    );
}

#[test]
fn _0036() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dt_duration_003",
        &ctx,
        r#"1"#,
    );
}

#[test]
fn _0037() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dt_duration_003_a",
        &ctx,
        r#"0"#,
    );
}

#[test]
fn _0038() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dt_duration_004",
        &ctx,
        r#"2"#,
    );
}

#[test]
fn _0039() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dt_duration_004_a",
        &ctx,
        r#"0"#,
    );
}

#[test]
fn _0040() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dt_duration_005",
        &ctx,
        r#"2"#,
    );
}

#[test]
fn _0041() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dt_duration_005_a",
        &ctx,
        r#"0"#,
    );
}

#[test]
fn _0042() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dt_duration_006",
        &ctx,
        r#"2"#,
    );
}

#[test]
fn _0043() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "dt_duration_006_a",
        &ctx,
        r#"0"#,
    );
}

#[test]
fn _0044() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_001",
        &ctx,
        r#"1"#,
    );
}

#[test]
fn _0045() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_002",
        &ctx,
        r#"10"#,
    );
}

#[test]
fn _0046() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_003",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0047() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_004",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0048() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_005",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0049() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_006",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0050() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_007",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0051() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_008",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0052() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_009",
        &ctx,
        r#"null(no such property in unary less: start)"#,
    );
}

#[test]
fn _0053() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_010",
        &ctx,
        r#"null(no such property in unary less or equal: start)"#,
    );
}

#[test]
fn _0054() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_011",
        &ctx,
        r#"10"#,
    );
}

#[test]
fn _0055() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_012",
        &ctx,
        r#"10"#,
    );
}

#[test]
fn _0056() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_013",
        &ctx,
        r#"10"#,
    );
}

#[test]
fn _0057() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_014",
        &ctx,
        r#"10"#,
    );
}

#[test]
fn _0058() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_015",
        &ctx,
        r#"null(no such property in unary greater: end)"#,
    );
}

#[test]
fn _0059() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_016",
        &ctx,
        r#"null(no such property in unary greater or equal: end)"#,
    );
}

#[test]
fn _0060() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_017",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0061() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_018",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0062() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_019",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0063() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_020",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0064() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_021",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0065() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_022",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0066() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_023",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0067() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "range_024",
        &ctx,
        r#"false"#,
    );
}
