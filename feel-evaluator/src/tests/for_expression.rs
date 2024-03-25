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
  te_be_value(false, &scope!(), r#"for n in [12,8,32] return n + 1"#, "[13,9,33]");
}

#[test]
fn _0002() {
  te_be_value(false, &scope!(), r#"for n in 1..10 return n - 1"#, "[0,1,2,3,4,5,6,7,8,9]");
}

#[test]
fn _0003() {
  let scope = &te_scope(r#"{N:4}"#);
  te_be_value(
    false,
    scope,
    r#"
      for i in 1..N return
        if i = 1 then
          1
        else
          i * partial[-1]
    "#,
    "[1,2,6,24]",
  );
}

#[test]
fn _0004() {
  let scope = &te_scope(r#"{N:5}"#);
  te_be_value(
    false,
    scope,
    r#"
      for i in 0..N return
        if i = 0 then
          1
        else if i = 1 then
          1 
        else 
          partial[-1] + partial[-2]
    "#,
    "[1,1,2,3,5,8]",
  );
}

#[test]
fn _0005() {
  te_be_value(
    false,
    &scope!(),
    r#"
      for x in [1,2,3],
          y in [6,7,8]
      return x + y
    "#,
    "[7,8,9,8,9,10,9,10,11]",
  );
}

#[test]
fn _0006() {
  te_be_value(
    false,
    &scope!(),
    r#"for x in [1,2,3], y in [6,7,8], z in [-1,-2,-3] return x + y + z"#,
    "[6,5,4,7,6,5,8,7,6,7,6,5,8,7,6,9,8,7,8,7,6,9,8,7,10,9,8]",
  );
}

#[test]
fn _0007() {
  te_be_value(false, &scope!(), r#"for n in 4..2 return n"#, "[4,3,2]");
}
