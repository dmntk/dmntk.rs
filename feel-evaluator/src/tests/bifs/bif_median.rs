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
  te_null(false, &scope!(), "median()", r#"expected 1+ parameters, actual number of parameters is 0"#);
}

#[test]
fn _0002() {
  te_null(false, &scope!(), "median([])", "");
}

#[test]
fn _0003() {
  te_null(false, &scope!(), "median(l:[])", r#"parameter 'list' not found"#);
}

#[test]
fn _0004() {
  te_null(false, &scope!(), "median(l:[1,2,3])", r#"parameter 'list' not found"#);
}

#[test]
fn _0005() {
  te_null(false, &scope!(), "median([true,false])", r#"median"#);
}

#[test]
fn _0006() {
  te_number(false, &scope!(), "median([8, 2, 5, 7])", 6, 0);
}

#[test]
fn _0007() {
  te_number(false, &scope!(), "median(list:[8, 2, 5, 7])", 6, 0);
}

#[test]
fn _0008() {
  te_number(false, &scope!(), "median([8,2,5,3,4])", 4, 0);
}

#[test]
fn _0009() {
  te_number(false, &scope!(), "median(list:[8,2,5,3,4])", 4, 0);
}

#[test]
fn _0010() {
  te_number(false, &scope!(), "median(8,2,5,3,4)", 4, 0);
}

#[test]
fn _0011() {
  te_number(false, &scope!(), "median([8,2,5,3,4.25])", 425, 2);
}

#[test]
fn _0012() {
  te_number(false, &scope!(), "median(list:[8,2,5,3,4.25])", 425, 2);
}

#[test]
fn _0013() {
  te_number(false, &scope!(), "median(8,2,5,3,4.25)", 425, 2);
}

#[test]
fn _0014() {
  te_number(false, &scope!(), "median([6,1,2,3])", 25, 1);
}

#[test]
fn _0015() {
  te_number(false, &scope!(), "median(list:[6,1,2,3])", 25, 1);
}

#[test]
fn _0016() {
  te_number(false, &scope!(), "median(6,1,2,3)", 25, 1);
}

#[test]
fn _0017() {
  te_number(false, &scope!(), "median([2021])", 2021, 0);
}

#[test]
fn _0018() {
  te_number(false, &scope!(), "median(list:[2021])", 2021, 0);
}

#[test]
fn _0019() {
  te_number(false, &scope!(), "median(2021)", 2021, 0);
}

#[test]
fn _0020() {
  te_number(false, &scope!(), "median([1999,2999])", 2499, 0);
}

#[test]
fn _0021() {
  te_number(false, &scope!(), "median(list:[1999,2999])", 2499, 0);
}

#[test]
fn _0022() {
  te_number(false, &scope!(), "median(1999,2999)", 2499, 0);
}
