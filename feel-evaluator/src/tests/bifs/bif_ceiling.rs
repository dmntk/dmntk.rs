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
  te_number(false, &scope!(), "ceiling(1.5)", 2, 0);
}

#[test]
fn _0002() {
  te_number(false, &scope!(), "ceiling(-1.5)", -1, 0);
}

#[test]
fn _0003() {
  te_number(false, &scope!(), "ceiling(--1)", 1, 0);
}

#[test]
fn _0004() {
  te_number(false, &scope!(), "ceiling(-5/2.3*5)", -10, 0);
}
#[test]
fn _0005() {
  te_number(false, &scope!(), "ceiling(n:5.777)", 6, 0);
}
#[test]
fn _0006() {
  te_number(false, &scope!(), "ceiling(n:-.33333)", 0, 0);
}
#[test]
fn _0007() {
  te_number(false, &scope!(), "ceiling(n:.33333)", 1, 0);
}

#[test]
fn _0008() {
  let scope = &te_scope("{ Order size: 23.27 }");
  te_number(false, scope, "ceiling(n:Order size)", 24, 0);
}

#[test]
fn _0009() {
  te_null(false, &scope!(), "ceiling(number:5.777)", r#"parameter 'n' not found"#);
}

#[test]
fn _0010() {
  te_null(
    false,
    &scope!(),
    r#"ceiling(true)"#,
    r#"[core::ceiling] invalid argument type, expected number, actual type is boolean"#,
  );
}

#[test]
fn _0011() {
  te_null(false, &scope!(), r#"ceiling()"#, r#"expected 1 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0012() {
  te_null(false, &scope!(), r#"ceiling(1,2)"#, r#"expected 1 parameters, actual number of parameters is 2"#);
}
