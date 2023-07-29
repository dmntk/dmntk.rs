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
use crate::lalr::TokenType::StartTextualExpressions;

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpressions,
    r#"1,2,3"#,
    r#"
       ExpressionList
       ├─ Numeric
       │  └─ `1.`
       ├─ Numeric
       │  └─ `2.`
       └─ Numeric
          └─ `3.`
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  scope.set_name("a".into());
  scope.set_name("b".into());
  scope.set_name("c".into());
  accept(
    &scope,
    StartTextualExpressions,
    r#"a,b,c"#,
    r#"
       ExpressionList
       ├─ Name
       │  └─ `a`
       ├─ Name
       │  └─ `b`
       └─ Name
          └─ `c`
    "#,
    false,
  );
}

#[test]
fn _0003() {
  let scope = scope!();
  scope.set_name("a".into());
  scope.set_name("b".into());
  scope.set_name("c".into());
  accept(
    &scope,
    StartTextualExpressions,
    r#"a+b,b-c,c**a,(["a","b","c"])"#,
    r#"
       ExpressionList
       ├─ Add
       │  ├─ Name
       │  │  └─ `a`
       │  └─ Name
       │     └─ `b`
       ├─ Sub
       │  ├─ Name
       │  │  └─ `b`
       │  └─ Name
       │     └─ `c`
       ├─ Exp
       │  ├─ Name
       │  │  └─ `c`
       │  └─ Name
       │     └─ `a`
       └─ List
          ├─ String
          │  └─ `a`
          ├─ String
          │  └─ `b`
          └─ String
             └─ `c`
    "#,
    false,
  );
}
