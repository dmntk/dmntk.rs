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
  te_null(false, &scope!(), "min()", r#"expected 1+ parameters, actual number of parameters is 0"#);
}

#[test]
fn _0002() {
  te_null(false, &scope!(), "min([])", r#""#);
}

#[test]
fn _0003() {
  te_null(false, &scope!(), "min(l:[])", r#"parameter 'list' not found"#);
}

#[test]
fn _0004() {
  te_null(false, &scope!(), "min(list:[])", r#""#);
}

#[test]
fn _0005() {
  te_null(false, &scope!(), "min(l:[1,2,3])", r#"parameter 'list' not found"#);
}

#[test]
fn _0006() {
  te_number(false, &scope!(), "min(list:[1,2,3])", 1, 0);
}

#[test]
fn _0007() {
  te_number(false, &scope!(), "min([2021])", 2021, 0);
}

#[test]
fn _0008() {
  te_number(false, &scope!(), "min(2021)", 2021, 0);
}

#[test]
fn _0009() {
  te_number(false, &scope!(), "min([1,2,3])", 1, 0);
}

#[test]
fn _0010() {
  te_number(false, &scope!(), "min(1,2,3)", 1, 0);
}

#[test]
fn _0011() {
  te_number(false, &scope!(), "min([8,4,2.89,3,2.89,5])", 289, 2);
}

#[test]
fn _0012() {
  te_number(false, &scope!(), "min(8,4,2.89,3,2.89,5)", 289, 2);
}

#[test]
fn _0013() {
  te_number(false, &scope!(), "min([2837465.9584,-39408573.456749])", -39408573456749, 6);
}

#[test]
fn _0014() {
  te_number(false, &scope!(), "min(2837465.9584,-39408573.456749)", -39408573456749, 6);
}

#[test]
fn _0015() {
  te_string(false, &scope!(), r#"min(["a","b","c"])"#, "a");
}

#[test]
fn _0016() {
  te_string(false, &scope!(), r#"min("b","a","c")"#, "a");
}

#[test]
fn _0017() {
  te_string(false, &scope!(), r#"min(["Johnny","John"])"#, "John");
}

#[test]
fn _0018() {
  te_string(false, &scope!(), r#"min("Johnny","John")"#, "John");
}

#[test]
fn _0019() {
  te_null(
    false,
    &scope!(),
    r#"min(1,2,3,"a")"#,
    r#"[core::min] invalid argument type, expected number, actual type is string"#,
  );
}

#[test]
fn _0020() {
  te_null(
    false,
    &scope!(),
    r#"min("a","b","c",true)"#,
    r#"[core::min] invalid argument type, expected string, actual type is boolean"#,
  );
}

#[test]
fn _0021() {
  te_null(
    false,
    &scope!(),
    r#"min(true, "a","b","c",1,2)"#,
    r#"[core::min] invalid argument type, expected number, string, actual type is boolean"#,
  );
}
