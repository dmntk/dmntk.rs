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
  te_be_value(false, &scope!(), r#"stddev(2,4,7,5)"#, r#"2.081665999466132735282297706979931"#);
}

#[test]
fn _0002() {
  te_number(false, &scope!(), r#"decimal(stddev(2,4,7,5),13)"#, 20816659994661, 13);
}

#[test]
fn _0003() {
  te_number(false, &scope!(), r#"decimal(stddev(2,4,7,5),9)"#, 2081665999, 9);
}

#[test]
fn _0004() {
  te_be_value(false, &scope!(), r#"stddev([2,4,7,5])"#, r#"2.081665999466132735282297706979931"#);
}

#[test]
fn _0005() {
  te_be_value(false, &scope!(), r#"stddev(list:[2,4,7,5])"#, r#"2.081665999466132735282297706979931"#);
}

#[test]
fn _0006() {
  te_be_value(false, &scope!(), r#"stddev(5,6,8,9)"#, r#"1.825741858350553711523232609336007"#);
}

#[test]
fn _0007() {
  te_null(false, &scope!(), r#"stddev()"#, r#"expected 1+ parameters, actual number of parameters is 0"#);
}

#[test]
fn _0008() {
  te_null(
    false,
    &scope!(),
    r#"stddev(true)"#,
    r#"[positional::stddev] invalid argument type, expected list, actual type is boolean"#,
  );
}

#[test]
fn _0009() {
  te_null(false, &scope!(), r#"stddev(l: [2,4,7,5])"#, r#"parameter 'list' not found"#);
}

#[test]
fn _0010() {
  te_null(false, &scope!(), r#"stddev([2])"#, r#"stddev: minimum two input arguments expected"#);
}

#[test]
fn _0011() {
  te_null(
    false,
    &scope!(),
    r#"stddev([2,"a","b"])"#,
    r#"stddev: expected number, actual type is string with value "a""#,
  );
}

#[test]
fn _0012() {
  te_null(
    false,
    &scope!(),
    r#"stddev([1, 4 ** 6174, 4 ** 6174, 4 ** 6174])"#,
    r#"stddev: intermediate result is not a finite number"#,
  );
}

#[test]
fn _0013() {
  te_null(false, &scope!(), r#"stddev([exp(7075),-exp(7074)])"#, r#"stddev: result is not a finite number"#);
}
