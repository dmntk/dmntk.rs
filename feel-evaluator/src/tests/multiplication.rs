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

use super::*;
use dmntk_feel::scope;

#[test]
fn _0001() {
  te_number(false, &scope!(), "1*1", 1, 0);
}

#[test]
fn _0002() {
  te_number(false, &scope!(), " 1 * 2 ", 2, 0);
}

#[test]
fn _0003() {
  te_number(false, &scope!(), " 5 *2 *3", 30, 0);
}

#[test]
fn _0004() {
  te_number(false, &scope!(), "10*2*5", 100, 0);
}

#[test]
fn _0005() {
  te_number(false, &scope!(), "( 1 * 2 ) * ( 3 * 4 )", 24, 0);
}

#[test]
fn _0006() {
  te_number(false, &scope!(), "( ( ( 4 * 3 ) ) )", 12, 0);
}

#[test]
fn _0007() {
  te_number(false, &scope!(), "(3*2)+(4*5)", 26, 0);
}

#[test]
fn _0008() {
  te_number(false, &scope!(), "3*2+4*5", 26, 0);
}

#[test]
fn _0009() {
  te_number(false, &scope!(), "3*(2+4)*5", 90, 0);
}

#[test]
fn _0010() {
  te_number(false, &scope!(), ".10 * 30.00", 3, 0);
}

#[test]
fn _0011() {
  te_number(false, &scope!(), "1.0*10**3", 1000, 0);
}

#[test]
fn _0012() {
  let scope = &te_scope("{Monthly Salary:10000}");
  te_number(false, scope, "12 * Monthly Salary", 120000, 0);
}

#[test]
fn _0013() {
  te_days_and_time_duration_x(false, &scope!(), r#" 1.5 * @"P4DT1H" "#, "P6DT1H30M");
}

#[test]
fn _0014() {
  te_days_and_time_duration_x(false, &scope!(), r#" @"P4DT1H" * 1.5 "#, "P6DT1H30M");
}

#[test]
fn _0015() {
  te_years_and_months_duration_x(false, &scope!(), r#" 10 * @"P1Y" "#, "P10Y");
}

#[test]
fn _0016() {
  te_years_and_months_duration_x(false, &scope!(), r#" @"P1Y" * 10 "#, "P10Y");
}

#[test]
fn _0017() {
  te_null(false, &scope!(), r#" 1 * "a" "#, r#"[multiplication] incompatible types: 1 * "a""#);
}

#[test]
fn _0018() {
  te_null(false, &scope!(), r#" @"P4DT1H" * @"P2Y" "#, r#"[multiplication] incompatible types: P4DT1H * P2Y"#);
}

#[test]
fn _0019() {
  te_null(false, &scope!(), r#" @"P2Y" * @"P4DT1H" "#, r#"[multiplication] incompatible types: P2Y * P4DT1H"#);
}

#[test]
fn _0020() {
  te_null(
    false,
    &scope!(),
    r#" 999999999999999999999 * @"P1D" "#,
    r#"multiplication result is out of range of days and time duration"#,
  );
}

#[test]
fn _0021() {
  te_null(
    false,
    &scope!(),
    r#" 999999999999999999999 * @"P1Y" "#,
    r#"multiplication result is out of range of years and months duration"#,
  );
}

#[test]
fn _0022() {
  te_null(
    false,
    &scope!(),
    r#" @"P1D" * 999999999999999999999 "#,
    r#"multiplication result is out of range of days and time duration"#,
  );
}

#[test]
fn _0023() {
  te_null(
    false,
    &scope!(),
    r#" @"P1Y" * 999999999999999999999 "#,
    r#"multiplication result is out of range of years and months duration"#,
  );
}

#[test]
fn _0024() {
  te_null(false, &scope!(), r#" null * @"P1Y" "#, r#""#);
}

#[test]
fn _0025() {
  te_null(false, &scope!(), r#" "a" * 2 "#, r#"unexpected value type in multiplication: string"#);
}
