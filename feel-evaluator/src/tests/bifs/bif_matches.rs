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
    te_bool(false, &scope!(), r#"matches("foobar","^fo*b")"#, true);
}

#[test]
fn _0002() {
    te_bool(
        false,
        &scope!(),
        r#"matches(input: "foobar", pattern: "^fo*b")"#,
        true,
    );
}

#[test]
fn _0003() {
    te_bool(false, &scope!(), r#"matches("abracadabra","bra")"#, true);
}

#[test]
fn _0004() {
    te_bool(false, &scope!(), r#"matches("abracadabra","^a.*a$")"#, true);
}

#[test]
fn _0005() {
    te_bool(false, &scope!(), r#"matches("abracadabra","^bra")"#, false);
}

#[test]
fn _0006() {
    te_bool(
        false,
        &scope!(),
        r#"matches("hello\nworld","hello.*world")"#,
        false,
    );
}

#[test]
fn _0007() {
    te_bool(
        false,
        &scope!(),
        r#"matches("hello\nworld","hello.*world","s")"#,
        true,
    );
}

#[test]
fn _0008() {
    te_bool(
        false,
        &scope!(),
        r#"matches(input: "hello\nworld", pattern: "hello.*world", flags: "s")"#,
        true,
    );
}

#[test]
fn _0009() {
    let scope = &te_scope(
        r#"{poem:"<poem author=\"Wilhelm Busch\">\nKaum hat dies der Hahn gesehen,\nFängt er auch schon an zu krähen:\nKikeriki! Kikikerikih!!\nTak, tak, tak! - da kommen sie.\n</poem>"}"#,
    );
    te_bool(false, scope, r#"matches(poem, "Kaum.*krähen")"#, false);
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"matches()"#,
        r#"expected 2,3 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0011() {
    te_null(
        false,
        &scope!(),
        r#"matches("abc")"#,
        r#"expected 2,3 parameters, actual number of parameters is 1"#,
    );
}

#[test]
fn _0012() {
    te_null(
        false,
        &scope!(),
        r#"matches("abc","a","b","c")"#,
        r#"expected 2,3 parameters, actual number of parameters is 4"#,
    );
}

#[test]
fn _0013() {
    te_null(
        false,
        &scope!(),
        r#"matches(i: "foobar", pattern: "^fo*b")"#,
        r#"parameter 'input' not found"#,
    );
}

#[test]
fn _0014() {
    te_null(
        false,
        &scope!(),
        r#"matches(input: "foobar", p: "^fo*b")"#,
        r#"parameter 'pattern' not found"#,
    );
}

#[test]
fn _0015() {
    te_null(
        false,
        &scope!(),
        r#"matches(input: 10, pattern: "^fo*b")"#,
        r#"matches"#,
    );
}

#[test]
fn _0016() {
    te_null(
        false,
        &scope!(),
        r#"matches(input: "foobar", pattern: true)"#,
        r#"matches"#,
    );
}
