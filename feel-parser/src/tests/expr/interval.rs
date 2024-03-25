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
use crate::context::ParsingContext;
use crate::lalr::TokenType::StartExpression;

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "[1..10]",
    r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1.`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `10.`
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
    "(1..10]",
    r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1.`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `10.`
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
    "[1..10)",
    r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1.`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `10.`
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
    "(1..10)",
    r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1.`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `10.`
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
    "]1..10]",
    r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1.`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `10.`
    "#,
    false,
  );
}

#[test]
fn _0006() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "[1..10[",
    r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1.`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `10.`
    "#,
    false,
  );
}

#[test]
fn _0007() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "]1..10[",
    r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1.`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `10.`
    "#,
    false,
  );
}

#[test]
fn _0008() {
  let scope = scope!();
  scope.set_name("a".into());
  scope.set_name("b".into());
  accept(
    &scope,
    StartExpression,
    "[a..b]",
    r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ QualifiedName
       │     └─ Name
       │        └─ `a`
       └─ IntervalEnd (closed)
          └─ QualifiedName
             └─ Name
                └─ `b`
    "#,
    false,
  );
}

#[test]
fn _0009() {
  let scope = scope!();
  let mut ctx = ParsingContext::default();
  ctx.set_name("start".into());
  ctx.set_name("end".into());
  scope.set_context("r".into(), ctx);
  accept(
    &scope,
    StartExpression,
    "[r.start..r.end]",
    r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ QualifiedName
       │     ├─ Name
       │     │  └─ `r`
       │     └─ Name
       │        └─ `start`
       └─ IntervalEnd (closed)
          └─ QualifiedName
             ├─ Name
             │  └─ `r`
             └─ Name
                └─ `end`
    "#,
    false,
  );
}

#[test]
fn _00010() {
  let scope = scope!();
  scope.set_name("r".into());
  scope.set_name("start".into());
  scope.set_name("end".into());
  accept(
    &scope,
    StartExpression,
    "[r.start..r.end]",
    r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ QualifiedName
       │     ├─ Name
       │     │  └─ `r`
       │     └─ Name
       │        └─ `start`
       └─ IntervalEnd (closed)
          └─ QualifiedName
             ├─ Name
             │  └─ `r`
             └─ Name
                └─ `end`
    "#,
    false,
  );
}

#[test]
fn _00011() {
  let scope = scope!();
  scope.set_name("r".into());
  scope.set_name("s".into());
  scope.set_name("start".into());
  scope.set_name("end".into());
  accept(
    &scope,
    StartExpression,
    "[r.start..r.s.end]",
    r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ QualifiedName
       │     ├─ Name
       │     │  └─ `r`
       │     └─ Name
       │        └─ `start`
       └─ IntervalEnd (closed)
          └─ QualifiedName
             ├─ Name
             │  └─ `r`
             ├─ Name
             │  └─ `s`
             └─ Name
                └─ `end`
    "#,
    false,
  );
}
