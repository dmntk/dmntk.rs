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
  te_bool(false, &scope!(), "odd(2)", false);
}

#[test]
fn _0002() {
  te_bool(false, &scope!(), "odd(-2)", false);
}

#[test]
fn _0003() {
  te_bool(false, &scope!(), "odd(1)", true);
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), "odd(-1)", true);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), "odd(0)", false);
}

#[test]
fn _0006() {
  te_bool(false, &scope!(), "odd(-0)", false);
}

#[test]
fn _0007() {
  te_null(false, &scope!(), "odd()", r#"expected 1 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0008() {
  te_null(false, &scope!(), "odd(4,4)", r#"expected 1 parameters, actual number of parameters is 2"#);
}

#[test]
fn _0009() {
  te_bool(false, &scope!(), "odd(number:4)", false);
}

#[test]
fn _0010() {
  te_null(false, &scope!(), "odd(n:4)", r#"parameter 'number' not found"#);
}

#[test]
fn _0011() {
  let scope = &te_scope("{ even number: 20, odd number: 21 }");
  te_bool(false, scope, "odd(even number)", false);
}

#[test]
fn _0012() {
  let scope = &te_scope("{ even number: 20, odd number: 21 }");
  te_bool(false, scope, "odd(odd number)", true);
}

#[test]
fn _0013() {
  te_null(
    false,
    &scope!(),
    r#"odd("2")"#,
    r#"[core::odd] invalid argument type, expected number, actual type is string"#,
  );
}
