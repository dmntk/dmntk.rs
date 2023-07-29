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

#[test]
fn _0001() {
  let scope = &te_scope("{}");
  te_be_value(false, scope, "log(4)", r#"1.386294361119890618834464242916353"#);
}

#[test]
fn _0002() {
  let scope = &te_scope("{}");
  te_null(false, scope, "log(-1)", "");
}

#[test]
fn _0003() {
  let scope = &te_scope("{}");
  te_null(false, scope, "log(0)", "");
}

#[test]
fn _0004() {
  let scope = &te_scope("{}");
  te_null(false, scope, "log(0.0)", "");
}

#[test]
fn _0005() {
  let scope = &te_scope("{}");
  te_null(false, scope, "log()", "expected 1 parameters, actual number of parameters is 0");
}

#[test]
fn _0006() {
  let scope = &te_scope("{}");
  te_null(false, scope, "log(4,4)", "expected 1 parameters, actual number of parameters is 2");
}

#[test]
fn _0007() {
  let scope = &te_scope("{}");
  te_be_value(false, scope, "log(number:4)", r#"1.386294361119890618834464242916353"#);
}

#[test]
fn _0008() {
  let scope = &te_scope("{}");
  te_null(false, scope, "log(n:4)", r#"parameter 'number' not found"#);
}

#[test]
fn _0009() {
  let scope = &te_scope("{}");
  te_null(false, scope, "log(null)", "");
}

#[test]
fn _0010() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"log("4")"#, "");
}

#[test]
fn _0011() {
  let scope = &te_scope("{}");
  te_null(false, scope, "log(true)", "");
}

#[test]
fn _0012() {
  let scope = &te_scope("{}");
  te_null(false, scope, "log(false)", "");
}

#[test]
fn _0013() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"log(duration("P4D"))"#, "");
}

#[test]
fn _0014() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"log(duration("P4Y"))"#, "");
}

#[test]
fn _0015() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"log(date("2018-12-06"))"#, "");
}

#[test]
fn _0016() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"log(time("00:00:00"))"#, "");
}

#[test]
fn _0017() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"log(date and time("2018-12-06T00:00:00"))"#, "");
}

#[test]
fn _0018() {
  let scope = &te_scope("{}");
  te_be_value(false, scope, "log(10)", r#"2.302585092994045684017991454684364"#);
}

#[test]
fn _0019() {
  let scope = &te_scope("{}");
  te_number(false, scope, "decimal(log(10),4)", 23026, 4);
}
