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

from_examples!(DMN_3_0085);

#[test]
fn _0001() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_001",
        r#"{}"#,
        r#""foo""#,
    );
}

#[test]
fn _0002() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decisionService_001",
        r#"{}"#,
        r#""foo""#,
    );
}

#[test]
fn _0003() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_002",
        r#"{decision_002_input: "baz"}"#,
        r#""foo baz""#,
    );
}

#[test]
fn _0004() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decisionService_002",
        r#"{decision_002_input: "baz"}"#,
        r#""foo baz""#,
    );
}

#[test]
fn _0005() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_002",
        r#"{}"#,
        r#""foo bar""#,
    );
}

#[test]
fn _0006() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decisionService_002",
        r#"{}"#,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0007() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_002",
        r#"{decision_002_input: null}"#,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0008() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decisionService_002",
        r#"{decision_002_input: null}"#,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0009() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_002",
        r#"{decision_002_input: 1234}"#,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0010() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decisionService_002",
        r#"{decision_002_input: 1234}"#,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0011() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_003",
        r#"{decision_003_input_1: "B", decision_003_input_2: "C", inputData_003: "D"}"#,
        r#""A B C D""#,
    );
}

#[test]
fn _0012() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decisionService_003",
        r#"{decision_003_input_1: "B", decision_003_input_2: "C", inputData_003: "D"}"#,
        r#""A B C D""#,
    );
}

#[test]
fn _0013() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_004_1",
        &ctx,
        r#""foo""#,
    );
}

#[test]
fn _0014() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decisionService_005",
        r#"{}"#,
        r#""foo""#,
    );
}

#[test]
fn _0015() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_005_1",
        r#"{}"#,
        r#"null(invalid number of arguments)"#,
    );
}

#[test]
fn _0016() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_005_2",
        r#"{}"#,
        r#""foo""#,
    );
}

#[test]
fn _0017() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_006_1",
        r#"{}"#,
        r#""foo bar""#,
    );
}

#[test]
fn _0018() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_007_1",
        r#"{}"#,
        r#"null(equal err 'null(after coercion)' =?= 'null')"#,
    );
}

#[test]
fn _0019() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decisionService_007",
        r#"{}"#,
        r#"true"#,
    );
}

#[test]
fn _0020() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_008_1",
        r#"{}"#,
        r#"null(invalid number of arguments)"#,
    );
}

#[test]
fn _0021() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_009_1",
        r#"{}"#,
        r#""foo bar""#,
    );
}

#[test]
fn _0022() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_010_1",
        r#"{}"#,
        r#"null(parameter with name decision_010_3 not found in arguments)"#,
    );
}

#[test]
fn _0023() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_011_1",
        r#"{}"#,
        r#""A B C D""#,
    );
}

#[test]
fn _0024() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_012_1",
        r#"{}"#,
        r#""A B C D""#,
    );
}

#[test]
fn _0025() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_013_1",
        r#"{}"#,
        r#"{decisionService_013: "A B", decision_013_3: "D", inputData_013_1: null}"#,
    );
}

#[test]
fn _0026() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "decision_014_1",
        r#"{}"#,
        r#"{decisionService_014: "A B", decision_014_3: "D", inputData_014_1: null}"#,
    );
}
