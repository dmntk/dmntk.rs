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
  let scope = scope!();
  te_number(false, &scope, "abs(0)", 0, 0);
}

#[test]
fn _0002() {
  let scope = scope!();
  te_number(false, &scope, "abs(-0)", 0, 0);
}

#[test]
fn _0003() {
  let scope = scope!();
  te_number(false, &scope, "abs(1)", 1, 0);
}

#[test]
fn _0004() {
  let scope = scope!();
  te_number(false, &scope, "abs(12.01)", 1201, 2);
}

#[test]
fn _0005() {
  let scope = scope!();
  te_number(false, &scope, "abs(-12.01)", 1201, 2);
}

#[test]
fn _0006() {
  let scope = scope!();
  te_number(false, &scope, "abs(-1)", 1, 0);
}

#[test]
fn _0007() {
  let scope = &scope!();
  te_number(false, scope, "abs(n:-34)", 34, 0);
}

#[test]
fn _0008() {
  let scope = &scope!();
  te_null(false, scope, "abs(number:-34)", r#"parameter 'n' not found"#);
}

#[test]
fn _0009() {
  let scope = &te_scope("{ Order size: -4.5 }");
  te_number(false, scope, "abs(Order size)", 45, 1);
}

#[test]
fn _0010() {
  te_days_and_time_duration(false, &scope!(), r#"abs(duration("-P1D"))"#, false, 24 * 60 * 60, 0);
}

#[test]
fn _0011() {
  te_years_and_months_duration(false, &scope!(), r#"abs(duration("-P1Y"))"#, 1, 0);
}

#[test]
fn _0012() {
  te_null(
    false,
    &scope!(),
    r#"abs(null)"#,
    r#"[core::abs] invalid argument type, expected number, actual type is Null"#,
  );
}

#[test]
fn _0013() {
  te_null(
    false,
    &scope!(),
    r#"abs(true)"#,
    r#"[core::abs] invalid argument type, expected number, actual type is boolean"#,
  );
}

#[test]
fn _0014() {
  te_null(
    false,
    &scope!(),
    r#"abs("-57")"#,
    r#"[core::abs] invalid argument type, expected number, actual type is string"#,
  );
}

#[test]
fn _0015() {
  te_null(false, &scope!(), r#"abs()"#, r#"expected 1 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0016() {
  te_null(false, &scope!(), r#"abs(1,2)"#, r#"expected 1 parameters, actual number of parameters is 2"#);
}
