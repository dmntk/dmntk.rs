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
  te_number(false, &scope!(), r#" 1/1 "#, 1, 0);
}

#[test]
fn _0002() {
  te_number(false, &scope!(), r#" 1 / 2 "#, 5, 1);
}

#[test]
fn _0003() {
  te_number(false, &scope!(), r#" 5 / 2 / 4 "#, 625, 3);
}

#[test]
fn _0004() {
  te_number(false, &scope!(), r#" 10 / 2 / 5"#, 1, 0);
}

#[test]
fn _0005() {
  te_number(false, &scope!(), r#"( 1 / 2 ) / ( 12 / 6 )"#, 25, 2);
}

#[test]
fn _0006() {
  te_number(false, &scope!(), r#"( ( ( 6 / 3 ) ) )"#, 2, 0);
}

#[test]
fn _0007() {
  te_number_x(false, &scope!(), r#"1/3"#, r#"0.3333333333333333333333333333333333"#);
}

#[test]
fn _0008() {
  te_number(false, &scope!(), r#"1.01/2"#, 505, 3);
}

#[test]
fn _0009() {
  te_null(false, &scope!(), r#"0.0 / 0.0"#, r#"[division] division by zero"#);
}

#[test]
fn _0010() {
  te_null(false, &scope!(), r#" 0.0 / "a" "#, r#"[division] incompatible types: 0.0 / "a""#);
}

#[test]
fn _0011() {
  te_days_and_time_duration_x(false, &scope!(), r#" @"P10D" / 2 "#, r#"P5D"#);
}

#[test]
fn _0012() {
  te_null(
    false,
    &scope!(),
    r#" @"P10D" / 0.00000000000000000000000000000000000000000000000001 "#,
    r#"[division] error: P10D / 0.00000000000000000000000000000000000000000000000001"#,
  );
}

#[test]
fn _0013() {
  te_null(false, &scope!(), r#" @"P10D" / 0 "#, r#"[division] division by zero"#);
}

#[test]
fn _0014() {
  te_number(false, &scope!(), r#" @"P10D" / @"P5D" "#, 2, 0);
}

#[test]
fn _0015() {
  te_null(false, &scope!(), r#" @"P10D" / @"P0D" "#, r#"[division] division by zero"#);
}

#[test]
fn _0016() {
  te_years_and_months_duration_x(false, &scope!(), r#" @"P10Y" / 2 "#, r#"P5Y"#);
}

#[test]
fn _0017() {
  te_null(
    false,
    &scope!(),
    r#" @"P10Y" / 0.00000000000000000000000000000000000000000000000001 "#,
    r#"[division] error: P10Y / 0.00000000000000000000000000000000000000000000000001"#,
  );
}

#[test]
fn _0018() {
  te_number(false, &scope!(), r#" @"P10Y" / @"P5Y" "#, 2, 0);
}

#[test]
fn _0019() {
  te_null(false, &scope!(), r#" @"P10Y" / 0 "#, r#"[division] division by zero"#);
}

#[test]
fn _0020() {
  te_null(false, &scope!(), r#" @"P10Y" / @"P0Y" "#, r#"[division] division by zero"#);
}

#[test]
fn _0021() {
  te_null(false, &scope!(), r#" @"P10D" / "a" "#, r#"[division] incompatible types: P10D / "a""#);
}

#[test]
fn _0022() {
  te_null(false, &scope!(), r#" @"P10Y" / "a" "#, r#"[division] incompatible types: P10Y / "a""#);
}

#[test]
fn _0023() {
  te_null(false, &scope!(), r#" "a" / 2.21 "#, r#"[division] incompatible types: "a" / 2.21"#);
}
