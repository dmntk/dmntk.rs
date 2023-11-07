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

use super::super::*;
use dmntk_feel::scope;

#[test]
fn _0001() {
    te_null(false, &scope!(), "string(null)", "");
}

#[test]
fn _0002() {
    te_string(false, &scope!(), "string(1.1)", "1.1");
}

#[test]
fn _0003() {
    te_string(false, &scope!(), "string(true)", "true");
}

#[test]
fn _0004() {
    te_string(false, &scope!(), "string(false)", "false");
}

#[test]
fn _0005() {
    te_string(
        false,
        &scope!(),
        "string({a:  1.0, b: 2.0})",
        "{a: 1.0, b: 2.0}",
    );
}

#[test]
fn _0006() {
    te_string(false, &scope!(), "string({a:  1, b: 2})", "{a: 1, b: 2}");
}

#[test]
fn _0007() {
    te_null(false, &scope!(), "string(from:null)", "");
}

#[test]
fn _0008() {
    te_string(false, &scope!(), "string(from:1.1)", "1.1");
}

#[test]
fn _0009() {
    te_string(false, &scope!(), r#"string(from:"Adam")"#, r#"Adam"#);
}

#[test]
fn _0010() {
    te_string(
        false,
        &scope!(),
        r#"string([1,2,3,"foo"])"#,
        r#"[1, 2, 3, "foo"]"#,
    );
}

#[test]
fn _0011() {
    te_string(
        false,
        &scope!(),
        r#"string([1,2,3,[4,5,"foo"]])"#,
        r#"[1, 2, 3, [4, 5, "foo"]]"#,
    );
}

#[test]
fn _0012() {
    te_string(false, &scope!(), r#"string(["\"foo\""])"#, r#"["\"foo\""]"#);
}

#[test]
fn _0013() {
    te_string(
        false,
        &scope!(),
        r#"string({"{": "foo"})"#,
        r#"{"{": "foo"}"#,
    );
}

#[test]
fn _0014() {
    te_string(
        false,
        &scope!(),
        r#"string({":": "foo"})"#,
        r#"{":": "foo"}"#,
    );
}

#[test]
fn _0015() {
    te_string(
        false,
        &scope!(),
        r#"string({",": "foo"})"#,
        r#"{",": "foo"}"#,
    );
}

#[test]
fn _0016() {
    te_string(
        false,
        &scope!(),
        r#"string({"}": "foo"})"#,
        r#"{"}": "foo"}"#,
    );
}

#[test]
fn _0017() {
    te_string(
        false,
        &scope!(),
        r#"string({"\"": "foo"})"#,
        r#"{"\"": "foo"}"#,
    );
}

#[test]
fn _0018() {
    te_null(
        false,
        &scope!(),
        "string()",
        r#"expected 1 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        "string(1,2)",
        r#"expected 1 parameters, actual number of parameters is 2"#,
    );
}

#[test]
fn _0020() {
    te_null(
        false,
        &scope!(),
        "string(f:1.1)",
        r#"parameter 'from' not found"#,
    );
}
