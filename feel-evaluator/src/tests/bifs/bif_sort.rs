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

#[test]
fn _0001() {
  let scope = &te_scope("{}");
  te_be_value(false, scope, "sort([3,1,4,5,2], function(x,y) x < y)", r#"[1,2,3,4,5]"#);
}

#[test]
fn _0002() {
  let scope = &te_scope("{}");
  te_be_value(false, scope, "sort(list: [3,1,4,5,2], precedes: function(x,y) x < y)", r#"[1,2,3,4,5]"#);
}

#[test]
fn _0003() {
  let scope = &te_scope("{}");
  te_null(
    false,
    scope,
    "sort([3,1,4,5,2], [10,11,12], function(x,y) x < y)",
    r#"expected 2 parameters, actual number of parameters is 3"#,
  );
}

#[test]
fn _0004() {
  let scope = &te_scope("{}");
  te_null(false, scope, "sort([3,1,4,5,2])", r#"expected 2 parameters, actual number of parameters is 1"#);
}

#[test]
fn _0005() {
  let scope = &te_scope("{}");
  te_null(false, scope, "sort()", r#"expected 2 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0006() {
  let scope = &te_scope("{}");
  te_null(false, scope, "sort(10,10)", r#"sort: expected a list of values as a first argument"#);
}

#[test]
fn _0007() {
  let scope = &te_scope("{}");
  te_null(
    false,
    scope,
    r#"sort([5,4,3,2,1],10)"#,
    r#"sort: expected ordering function definition as a second argument"#,
  );
}

#[test]
fn _0008() {
  let scope = &te_scope("{}");
  te_null(
    false,
    scope,
    r#"sort([3,1,4,5,2], function(x,y,z) x < y + z)"#,
    r#"sort: ordering function should take exactly two arguments"#,
  );
}

#[test]
fn _0009() {
  let scope = &te_scope("{}");
  te_null(
    false,
    scope,
    r#"sort([3,1,4,5,2], function(x) x < 10)"#,
    r#"sort: ordering function should take exactly two arguments"#,
  );
}

#[test]
fn _0010() {
  let scope = &te_scope("{}");
  te_null(
    false,
    scope,
    r#"sort([3,1,4,5,2], function() false)"#,
    r#"sort: ordering function should take exactly two arguments"#,
  );
}

#[test]
fn _0011() {
  let scope = &te_scope("{}");
  te_be_value(false, scope, r#"sort(["a","9","A","10","Aa","8"], function(x,y) x < y)"#, r#"["10","8","9","A","Aa","a"]"#);
}

#[test]
fn _0012() {
  let scope = &te_scope("{}");
  te_null(false, scope, "sort(l: [3,1,4,5,2], precedes: function(x,y) x < y)", r#"parameter 'list' not found"#);
}

#[test]
fn _0013() {
  let scope = &te_scope("{}");
  te_null(false, scope, "sort(list: [3,1,4,5,2], p: function(x,y) x < y)", r#"parameter 'precedes' not found"#);
}

#[test]
fn _0014() {
  let scope = &te_scope("{}");
  te_be_value(false, scope, "sort([3,1,4,5,2], function(x,y) 10)", r#"[3,1,4,5,2]"#);
}
