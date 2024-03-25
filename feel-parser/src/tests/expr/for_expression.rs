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
use crate::lalr::TokenType::StartExpression;

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"for n in 1..10 return n + 1"#,
    r#"
       For
       ├─ IterationContexts
       │  └─ IterationContextRange
       │     ├─ Name
       │     │  └─ `n`
       │     ├─ Numeric
       │     │  └─ `1.`
       │     └─ Numeric
       │        └─ `10.`
       └─ EvaluatedExpression
          └─ Add
             ├─ Name
             │  └─ `n`
             └─ Numeric
                └─ `1.`
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"for n in [1,2,3] return n + 1"#,
    r#"
       For
       ├─ IterationContexts
       │  └─ IterationContextSingle
       │     ├─ Name
       │     │  └─ `n`
       │     └─ List
       │        ├─ Numeric
       │        │  └─ `1.`
       │        ├─ Numeric
       │        │  └─ `2.`
       │        └─ Numeric
       │           └─ `3.`
       └─ EvaluatedExpression
          └─ Add
             ├─ Name
             │  └─ `n`
             └─ Numeric
                └─ `1.`
    "#,
    false,
  );
}

#[test]
fn _0003() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"for n in 1..10, m in 100..200 return n + m"#,
    r#"
       For
       ├─ IterationContexts
       │  ├─ IterationContextRange
       │  │  ├─ Name
       │  │  │  └─ `n`
       │  │  ├─ Numeric
       │  │  │  └─ `1.`
       │  │  └─ Numeric
       │  │     └─ `10.`
       │  └─ IterationContextRange
       │     ├─ Name
       │     │  └─ `m`
       │     ├─ Numeric
       │     │  └─ `100.`
       │     └─ Numeric
       │        └─ `200.`
       └─ EvaluatedExpression
          └─ Add
             ├─ Name
             │  └─ `n`
             └─ Name
                └─ `m`
    "#,
    false,
  );
}

#[test]
fn _0004() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"for n in [1,5,8], m in [15,18,-3] return n * m"#,
    r#"
       For
       ├─ IterationContexts
       │  ├─ IterationContextSingle
       │  │  ├─ Name
       │  │  │  └─ `n`
       │  │  └─ List
       │  │     ├─ Numeric
       │  │     │  └─ `1.`
       │  │     ├─ Numeric
       │  │     │  └─ `5.`
       │  │     └─ Numeric
       │  │        └─ `8.`
       │  └─ IterationContextSingle
       │     ├─ Name
       │     │  └─ `m`
       │     └─ List
       │        ├─ Numeric
       │        │  └─ `15.`
       │        ├─ Numeric
       │        │  └─ `18.`
       │        └─ Neg
       │           └─ Numeric
       │              └─ `3.`
       └─ EvaluatedExpression
          └─ Mul
             ├─ Name
             │  └─ `n`
             └─ Name
                └─ `m`
    "#,
    false,
  );
}

#[test]
fn _0005() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"for n in [1,5,8], m in [15,18,-3] return (n + 2) * (m -4)"#,
    r#"
       For
       ├─ IterationContexts
       │  ├─ IterationContextSingle
       │  │  ├─ Name
       │  │  │  └─ `n`
       │  │  └─ List
       │  │     ├─ Numeric
       │  │     │  └─ `1.`
       │  │     ├─ Numeric
       │  │     │  └─ `5.`
       │  │     └─ Numeric
       │  │        └─ `8.`
       │  └─ IterationContextSingle
       │     ├─ Name
       │     │  └─ `m`
       │     └─ List
       │        ├─ Numeric
       │        │  └─ `15.`
       │        ├─ Numeric
       │        │  └─ `18.`
       │        └─ Neg
       │           └─ Numeric
       │              └─ `3.`
       └─ EvaluatedExpression
          └─ Mul
             ├─ Add
             │  ├─ Name
             │  │  └─ `n`
             │  └─ Numeric
             │     └─ `2.`
             └─ Sub
                ├─ Name
                │  └─ `m`
                └─ Numeric
                   └─ `4.`
    "#,
    false,
  );
}

#[test]
fn _0006() {
  let scope = scope!();
  scope.set_name("N".into());
  accept(
    &scope,
    StartExpression,
    r#"for i in 0..N return if i = 0 then 1 else i * partial[-1]"#,
    r#"
       For
       ├─ IterationContexts
       │  └─ IterationContextRange
       │     ├─ Name
       │     │  └─ `i`
       │     ├─ Numeric
       │     │  └─ `0.`
       │     └─ Name
       │        └─ `N`
       └─ EvaluatedExpression
          └─ If
             ├─ Eq
             │  ├─ Name
             │  │  └─ `i`
             │  └─ Numeric
             │     └─ `0.`
             ├─ Numeric
             │  └─ `1.`
             └─ Mul
                ├─ Name
                │  └─ `i`
                └─ Filter
                   ├─ Name
                   │  └─ `partial`
                   └─ Neg
                      └─ Numeric
                         └─ `1.`
    "#,
    false,
  );
}

#[test]
fn _0007() {
  // This test is not correct syntactically, because the variable `partial` is a list,
  // but only this way it is well tested, if this variable is properly defined
  // in local context during parsing the `return` expression.
  let scope = scope!();
  scope.set_name("N".into());
  accept(
    &scope,
    StartExpression,
    r#"for i in 0..N return if i = 0 then 1 else i * (partial + 1)"#,
    r#"
       For
       ├─ IterationContexts
       │  └─ IterationContextRange
       │     ├─ Name
       │     │  └─ `i`
       │     ├─ Numeric
       │     │  └─ `0.`
       │     └─ Name
       │        └─ `N`
       └─ EvaluatedExpression
          └─ If
             ├─ Eq
             │  ├─ Name
             │  │  └─ `i`
             │  └─ Numeric
             │     └─ `0.`
             ├─ Numeric
             │  └─ `1.`
             └─ Mul
                ├─ Name
                │  └─ `i`
                └─ Add
                   ├─ Name
                   │  └─ `partial`
                   └─ Numeric
                      └─ `1.`
    "#,
    false,
  );
}
