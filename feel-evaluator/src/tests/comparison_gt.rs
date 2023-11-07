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
use dmntk_feel::scope;

#[test]
fn _0001() {
  te_bool(false, &scope!(), r#"2>1"#, true);
}

#[test]
fn _0002() {
  te_null(false, &scope!(), r#" 2 > "a" "#, "eval_greater_then_number");
}

#[test]
fn _0003() {
  te_null(false, &scope!(), r#" false > 21 "#, "eval_greater_then");
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), r#"1.277>1.276"#, true);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), r#".54635>-.54635"#, true);
}

#[test]
fn _0006() {
  te_bool(false, &scope!(), r#"2>3"#, false);
}

#[test]
fn _0007() {
  te_bool(false, &scope!(), r#"(1+1.1)>2.0"#, true);
}

#[test]
fn _0008() {
  te_bool(false, &scope!(), r#"(1.1+2)>3.11"#, false);
}

#[test]
fn _0009() {
  te_bool(false, &scope!(), r#" ( 1 + 0.99 ) > 1.9"#, true);
}

#[test]
fn _0010() {
  te_bool(false, &scope!(), r#" ( ( ( 1.1 + 3.1 ) ) ) > 4.21"#, false);
}

#[test]
fn _0011() {
  te_bool(false, &scope!(), r#"(0.9+1)>(5.1-3.3)"#, true);
}

#[test]
fn _0012() {
  te_bool(false, &scope!(), r#"(1*2)>(10.0/5.1)"#, true);
}

#[test]
fn _0013() {
  te_bool(false, &scope!(), r#" "alfa" > "alfa" "#, false);
}

#[test]
fn _0014() {
  te_null(false, &scope!(), r#" "alfa" > 1 "#, "eval_greater_then_string");
}

#[test]
fn _0015() {
  te_bool(false, &scope!(), r#" "beta" > "alfa" "#, true);
}

#[test]
fn _0016() {
  te_bool(false, &scope!(), r#" "alfa" > "beta" "#, false);
}

#[test]
fn _0017() {
  te_bool(false, &scope!(), r#"@"2021-11-10" > @"2021-11-09""#, true);
}

#[test]
fn _0018() {
  te_null(false, &scope!(), r#" @"2021-11-10" > "a" "#, "eval_greater_then_date");
}

#[test]
fn _0019() {
  te_bool(false, &scope!(), r#"@"2021-11-10" > @"2021-11-11""#, false);
}

#[test]
fn _0020() {
  te_bool(false, &scope!(), r#"@"2021-11-10" > @"2021-11-10""#, false);
}

#[test]
fn _0021() {
  te_bool(false, &scope!(), r#" @"2021-11-01T11:10:12" > @"2021-11-01T11:10:12" "#, false);
}

#[test]
fn _0022() {
  te_null(false, &scope!(), r#" @"2021-11-01T11:10:12" > "2021-11-01T11:10:12" "#, "eval_greater_then_date_time");
}

#[test]
fn _0023() {
  te_bool(false, &scope!(), r#" @"2021-11-01T11:10:13" > @"2021-11-01T11:10:12" "#, true);
}

#[test]
fn _0024() {
  te_bool(false, &scope!(), r#" @"2021-11-01T11:10:11" > @"2021-11-01T11:10:12" "#, false);
}

#[test]
fn _0025() {
  te_bool(false, &scope!(), r#" @"11:10:12" > @"11:10:12" "#, false);
}

#[test]
fn _0026() {
  te_bool(false, &scope!(), r#" @"10:10:10.23" > @"10:10:10.22" "#, true);
}

#[test]
fn _0027() {
  te_null(false, &scope!(), r#" @"11:10:13" > "11:10:12" "#, "eval_greater_then_time");
}

#[test]
fn _0028() {
  te_bool(false, &scope!(), r#" @"11:10:12" > @"11:10:12" "#, false);
}

#[test]
fn _0029() {
  te_bool(false, &scope!(), r#" @"11:10:11" > @"11:10:12" "#, false);
}

#[test]
fn _0030() {
  te_bool(false, &scope!(), r#" @"P1D" > @"P1D" "#, false);
}

#[test]
fn _0031() {
  te_null(false, &scope!(), r#" @"P1D" > "P1D" "#, "eval_greater_days_and_time_duration");
}

#[test]
fn _0032() {
  te_bool(false, &scope!(), r#" @"P2D" > @"P1D" "#, true);
}

#[test]
fn _0033() {
  te_bool(false, &scope!(), r#" @"P1D" > @"P2D" "#, false);
}

#[test]
fn _0034() {
  te_bool(false, &scope!(), r#" @"P1Y" > @"P1Y" "#, false);
}

#[test]
fn _0035() {
  te_null(false, &scope!(), r#" @"P1Y" > "P1Y" "#, "eval_greater_years_and_months_duration");
}

#[test]
fn _0036() {
  te_bool(false, &scope!(), r#" @"P2Y" > @"P1Y" "#, true);
}

#[test]
fn _0037() {
  te_bool(false, &scope!(), r#" @"P1Y" > @"P2Y" "#, false);
}
