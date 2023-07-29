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
  te_number(false, &scope!(), r#"number("1",",",".")"#, 1, 0);
}

#[test]
fn _0002() {
  te_number(false, &scope!(), r#"number(from: "1", grouping separator: ",", decimal separator: ".")"#, 1, 0);
}

#[test]
fn _0003() {
  te_number(false, &scope!(), r#"number("1,000.21",",",".")"#, 100021, 2);
}

#[test]
fn _0004() {
  te_number(false, &scope!(), r#"number("1 000.21"," ",".")"#, 100021, 2);
}

#[test]
fn _0005() {
  te_number(false, &scope!(), r#"number("1.000,21",".",",")"#, 100021, 2);
}

#[test]
fn _0006() {
  te_number(false, &scope!(), r#"number("12345",null,null)"#, 12345, 0);
}

#[test]
fn _0007() {
  te_number(false, &scope!(), r#"number("12,345",",",null)"#, 12345, 0);
}

#[test]
fn _0008() {
  te_number(false, &scope!(), r#"number("123,45",null,",")"#, 12345, 2);
}

#[test]
fn _0009() {
  te_null(
    false,
    &scope!(),
    r#"number("1,000.21",".",".")"#,
    r#"[core::number] decimal separator must be different from grouping separator"#,
  );
}

#[test]
fn _0010() {
  te_null(
    false,
    &scope!(),
    r#"number("1$000.21","$",".")"#,
    r#"[core::number] grouping separator must be space, period, comma or null"#,
  );
}

#[test]
fn _0011() {
  te_null(
    false,
    &scope!(),
    r#"number("1,000$21",",","$")"#,
    r#"[core::number] decimal separator must be period, comma or null"#,
  );
}

#[test]
fn _0012() {
  te_null(
    false,
    &scope!(),
    r#"number("123a56",null,null)"#,
    r#"[core::number] <FeelNumberError> invalid number literal '123a56'"#,
  );
}

#[test]
fn _0013() {
  te_null(
    false,
    &scope!(),
    r#"number("1,000.21",2,".")"#,
    r#"[core::number] grouping separator must be space, period, comma or null"#,
  );
}

#[test]
fn _0014() {
  te_null(
    false,
    &scope!(),
    r#"number("1,000.21",",",true)"#,
    r#"[core::number] decimal separator must be period, comma or null"#,
  );
}

#[test]
fn _0015() {
  te_null(
    false,
    &scope!(),
    r#"number(1000,null,null)"#,
    r#"[core::number] invalid argument type, expected string, actual type is number"#,
  );
}

#[test]
fn _0016() {
  te_null(false, &scope!(), r#"number()"#, r#"expected 3 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0017() {
  te_null(false, &scope!(), r#"number(1000)"#, r#"expected 3 parameters, actual number of parameters is 1"#);
}

#[test]
fn _0018() {
  te_null(false, &scope!(), r#"number(1000,",")"#, r#"expected 3 parameters, actual number of parameters is 2"#);
}

#[test]
fn _0019() {
  te_null(
    false,
    &scope!(),
    r#"number(1000,",",".",",")"#,
    r#"expected 3 parameters, actual number of parameters is 4"#,
  );
}

#[test]
fn _0020() {
  te_null(
    false,
    &scope!(),
    r#"number(f: "1", grouping separator: ",", decimal separator: ".")"#,
    r#"parameter 'from' not found"#,
  );
}

#[test]
fn _0021() {
  te_null(
    false,
    &scope!(),
    r#"number(from: "1", grouping sep: ",", decimal separator: ".")"#,
    r#"parameter 'grouping separator' not found"#,
  );
}

#[test]
fn _0022() {
  te_null(
    false,
    &scope!(),
    r#"number(from: "1", grouping separator: ",", decimal sep: ".")"#,
    r#"parameter 'decimal separator' not found"#,
  );
}
