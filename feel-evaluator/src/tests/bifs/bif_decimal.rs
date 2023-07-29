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

use super::super::*;
use dmntk_feel::scope;

#[test]
fn _0001() {
  te_number(false, &scope!(), "decimal(1,2)", 100, 2);
}

#[test]
fn _0002() {
  te_number(false, &scope!(), "decimal(n: 1, scale: 2)", 100, 2);
}

#[test]
fn _0003() {
  te_number(false, &scope!(), "decimal(scale: 2, n: 1)", 100, 2);
}

#[test]
fn _0004() {
  te_number(false, &scope!(), "decimal(1/3,2)", 330, 3);
}

#[test]
fn _0005() {
  te_number(false, &scope!(), "decimal(0.505,2)", 50, 2);
}

#[test]
fn _0006() {
  te_number(false, &scope!(), "decimal(0.515,2)", 52, 2);
}

#[test]
fn _0007() {
  te_number(false, &scope!(), "decimal(1/3, 2.5)", 33, 2);
}

#[test]
fn _0008() {
  te_null(false, &scope!(), "decimal(1/3, 6177)", "[core::decimal] scale is out of range: 6177");
}

#[test]
fn _0009() {
  te_null(false, &scope!(), "decimal(1/3, -6112)", "[core::decimal] scale is out of range: -6112");
}

#[test]
fn _0010() {
  te_null(false, &scope!(), r#"decimal(1/3, "scale")"#, r#"[core::decimal] scale value is not a number: "scale""#);
}

#[test]
fn _0011() {
  te_null(false, &scope!(), r#"decimal("number", 6)"#, r#"[core::decimal] number value is not a number: "number""#);
}

#[test]
fn _0012() {
  te_null(false, &scope!(), r#"decimal()"#, r#"expected 2 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0013() {
  te_null(false, &scope!(), r#"decimal("1234")"#, r#"expected 2 parameters, actual number of parameters is 1"#);
}

#[test]
fn _0014() {
  te_null(false, &scope!(), r#"decimal("1234",1,2)"#, r#"expected 2 parameters, actual number of parameters is 3"#);
}

#[test]
fn _0015() {
  te_null(false, &scope!(), "decimal(n: 1, s: 2)", r#"parameter 'scale' not found"#);
}

#[test]
fn _0016() {
  te_null(false, &scope!(), "decimal(number: 1, scale: 2)", r#"parameter 'n' not found"#);
}
