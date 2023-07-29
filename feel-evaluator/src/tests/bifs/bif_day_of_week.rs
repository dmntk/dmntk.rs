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
  te_string(false, &scope!(), r#"day of week(date(2019,9,17))"#, r#"Tuesday"#);
}

#[test]
fn _0002() {
  te_string(false, &scope!(), r#"day of week(date and time("2019-09-17T00:00:00"))"#, r#"Tuesday"#);
}

#[test]
fn _0003() {
  te_string(false, &scope!(), r#"day of week(date and time("2019-09-17T23:59:59"))"#, r#"Tuesday"#);
}

#[test]
fn _0004() {
  te_string(false, &scope!(), r#"day of week(date(2022,2,7))"#, r#"Monday"#);
}

#[test]
fn _0005() {
  te_string(false, &scope!(), r#"day of week(date and time("2022-02-07T12:00:00.0"))"#, r#"Monday"#);
}

#[test]
fn _0006() {
  te_string(false, &scope!(), r#"day of week(date(2022,2,8))"#, r#"Tuesday"#);
}

#[test]
fn _0007() {
  te_string(false, &scope!(), r#"day of week(date and time("2022-02-08T12:00:00.0"))"#, r#"Tuesday"#);
}

#[test]
fn _0008() {
  te_string(false, &scope!(), r#"day of week(date(2022,2,9))"#, r#"Wednesday"#);
}

#[test]
fn _0009() {
  te_string(false, &scope!(), r#"day of week(date and time("2022-02-09T12:00:00.0"))"#, r#"Wednesday"#);
}

#[test]
fn _0010() {
  te_string(false, &scope!(), r#"day of week(date(2022,2,10))"#, r#"Thursday"#);
}

#[test]
fn _0011() {
  te_string(false, &scope!(), r#"day of week(date and time("2022-02-10T12:00:00.0"))"#, r#"Thursday"#);
}

#[test]
fn _0012() {
  te_string(false, &scope!(), r#"day of week(date(2022,2,11))"#, r#"Friday"#);
}

#[test]
fn _0013() {
  te_string(false, &scope!(), r#"day of week(date and time("2022-02-11T12:00:00.0"))"#, r#"Friday"#);
}

#[test]
fn _0014() {
  te_string(false, &scope!(), r#"day of week(date(2022,2,12))"#, r#"Saturday"#);
}

#[test]
fn _0015() {
  te_string(false, &scope!(), r#"day of week(date and time("2022-02-12T12:00:00.0"))"#, r#"Saturday"#);
}

#[test]
fn _0016() {
  te_string(false, &scope!(), r#"day of week(date(2022,2,13))"#, r#"Sunday"#);
}

#[test]
fn _0017() {
  te_string(false, &scope!(), r#"day of week(date and time("2022-02-13T12:00:00.0"))"#, r#"Sunday"#);
}

#[test]
fn _0018() {
  te_string(false, &scope!(), r#"day of week(date: date(2019,9,17))"#, r#"Tuesday"#);
}

#[test]
fn _0019() {
  te_null(
    false,
    &scope!(),
    r#"day of week(date(2019,2,30))"#,
    r#"[core::day of week] invalid argument type, expected date, date and time, actual type is Null"#,
  );
}

#[test]
fn _0020() {
  te_null(false, &scope!(), r#"day of week(date(999999999,1,1))"#, r#"[day of week] no weekday"#);
}

#[test]
fn _0021() {
  te_null(false, &scope!(), r#"day of week(d: date(2019,1,15))"#, r#"parameter 'date' not found"#);
}

#[test]
fn _0022() {
  te_null(false, &scope!(), r#"day of week()"#, r#"expected 1 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0023() {
  te_null(
    false,
    &scope!(),
    r#"day of week(date(2019,1,15),date(2019,1,15))"#,
    r#"expected 1 parameters, actual number of parameters is 2"#,
  );
}
