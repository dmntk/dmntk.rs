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
  let scope = scope!();
  te_bool(false, &scope, "all(true)", true);
}

#[test]
fn _0002() {
  let scope = scope!();
  te_bool(false, &scope, "all(true,true)", true);
}

#[test]
fn _0003() {
  let scope = scope!();
  te_bool(false, &scope, "all(true,true,true)", true);
}

#[test]
fn _0004() {
  let scope = scope!();
  te_bool(false, &scope, "all(false)", false);
}

#[test]
fn _0005() {
  let scope = scope!();
  te_bool(false, &scope, "all(true,false)", false);
}

#[test]
fn _0006() {
  let scope = scope!();
  te_bool(false, &scope, "all(false,true,true)", false);
}

#[test]
fn _0007() {
  let scope = scope!();
  te_null(false, &scope, "all(null,true,true)", r#""#);
}

#[test]
fn _0008() {
  let scope = scope!();
  te_bool(false, &scope, "all(false,null,true)", false);
}

#[test]
fn _0009() {
  let scope = scope!();
  te_bool(false, &scope, "all([])", true);
}

#[test]
fn _0010() {
  let scope = scope!();
  te_bool(false, &scope, "all(list: [])", true);
}

#[test]
fn _0011() {
  let scope = scope!();
  te_bool(false, &scope, "all([true])", true);
}

#[test]
fn _0012() {
  let scope = scope!();
  te_bool(false, &scope, "all(list: [true])", true);
}

#[test]
fn _0013() {
  let scope = scope!();
  te_bool(false, &scope, "all([true,true])", true);
}

#[test]
fn _0014() {
  let scope = scope!();
  te_bool(false, &scope, "all([true,true,true])", true);
}

#[test]
fn _0015() {
  let scope = scope!();
  te_bool(false, &scope, "all([false])", false);
}

#[test]
fn _0016() {
  let scope = scope!();
  te_bool(false, &scope, "all([true,false])", false);
}

#[test]
fn _0017() {
  let scope = scope!();
  te_bool(false, &scope, "all([false,true,true])", false);
}

#[test]
fn _0018() {
  let scope = scope!();
  te_null(false, &scope, "all([null,true,true])", r#""#);
}

#[test]
fn _0019() {
  let scope = scope!();
  te_bool(false, &scope, "all([false,null,true])", false);
}

#[test]
fn _0020() {
  let scope = scope!();
  te_null(false, &scope, "all([1])", r#""#);
}

#[test]
fn _0021() {
  let scope = scope!();
  te_null(false, &scope, "all([true,1])", r#""#);
}

#[test]
fn _0022() {
  let scope = scope!();
  te_bool(false, &scope, "all([false,1])", false);
}

#[test]
fn _0023() {
  let scope = scope!();
  te_null(false, &scope, "all([1,true,true])", r#""#);
}

#[test]
fn _0024() {
  let scope = scope!();
  te_null(false, &scope, "all()", r#"expected 1+ parameters, actual number of parameters is 0"#);
}

#[test]
fn _0025() {
  let scope = scope!();
  te_null(false, &scope, "all(l: [true,true,true])", r#"parameter 'list' not found"#);
}
