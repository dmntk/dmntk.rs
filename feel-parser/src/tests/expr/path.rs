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
    r#"1.first"#,
    r#"
       Path
       ├─ Numeric
       │  └─ `1.`
       └─ Name
          └─ `first`
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  scope.set_name("Manager".into());
  accept(
    &scope,
    StartExpression,
    r#"Manager.Name"#,
    r#"
       Path
       ├─ Name
       │  └─ `Manager`
       └─ Name
          └─ `Name`
    "#,
    false,
  );
}

#[test]
fn _0003() {
  let scope = scope!();
  scope.set_name("Manager".into());
  accept(
    &scope,
    StartExpression,
    r#"Manager.Address.Street"#,
    r#"
       Path
       ├─ Name
       │  └─ `Manager`
       └─ Name
          └─ `Address.Street`
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
    r#"(Manager.Address).Street"#,
    r#"
       Path
       ├─ Name
       │  └─ `Manager.Address`
       └─ Name
          └─ `Street`
    "#,
    false,
  );
}

#[test]
fn _0005() {
  let scope = scope!();
  scope.set_name("Manager".into());
  scope.set_name("Address".into());
  accept(
    &scope,
    StartExpression,
    r#"Manager.Address.Street"#,
    r#"
       Path
       ├─ Name
       │  └─ `Manager`
       └─ Path
          ├─ Name
          │  └─ `Address`
          └─ Name
             └─ `Street`
    "#,
    false,
  );
}

#[test]
fn _0006() {
  let scope = scope!();
  scope.set_name("Manager".into());
  scope.set_name("Address".into());
  scope.set_name("City".into());
  scope.set_name("Street".into());
  accept(
    &scope,
    StartExpression,
    r#"Manager.Address.City.Street"#,
    r#"
       Path
       ├─ Name
       │  └─ `Manager`
       └─ Path
          ├─ Name
          │  └─ `Address`
          └─ Path
             ├─ Name
             │  └─ `City`
             └─ Name
                └─ `Street`
    "#,
    false,
  );
}

#[test]
fn _0007() {
  let scope = scope!();
  let mut ctx_a = ParsingContext::default();
  ctx_a.set_name("Street".into());
  let mut ctx_b = ParsingContext::default();
  ctx_b.set_context("Address".into(), ctx_a);
  scope.set_context("Manager".into(), ctx_b);
  accept(
    &scope,
    StartExpression,
    r#"Manager.Address.Street"#,
    r#"
       Path
       ├─ Name
       │  └─ `Manager`
       └─ Path
          ├─ Name
          │  └─ `Address`
          └─ Name
             └─ `Street`
    "#,
    false,
  );
}

#[test]
fn _0008() {
  let scope = scope!();
  let mut ctx_1 = ParsingContext::default();
  ctx_1.set_name("principal".into());
  scope.set_context("loan".into(), ctx_1);
  accept(
    &scope,
    StartExpression,
    r#"(loan.principal)"#,
    r#"
       Path
       ├─ Name
       │  └─ `loan`
       └─ Name
          └─ `principal`
    "#,
    false,
  );
}

#[test]
fn _0009() {
  let scope = scope!();
  scope.set_name("principal".into());
  scope.set_name("loan".into());
  scope.set_name("id".into());
  accept(
    &scope,
    StartExpression,
    r#"(loan.principal.id)"#,
    r#"
       Path
       ├─ Name
       │  └─ `loan`
       └─ Path
          ├─ Name
          │  └─ `principal`
          └─ Name
             └─ `id`
    "#,
    false,
  );
}

#[test]
fn _0010() {
  let scope = scope!();
  scope.set_name("principal".into());
  scope.set_name("loan".into());
  scope.set_name("id".into());
  scope.set_name("type".into());
  accept(
    &scope,
    StartExpression,
    r#"(loan.principal.id.type - 1)"#,
    r#"
       Sub
       ├─ Path
       │  ├─ Name
       │  │  └─ `loan`
       │  └─ Path
       │     ├─ Name
       │     │  └─ `principal`
       │     └─ Path
       │        ├─ Name
       │        │  └─ `id`
       │        └─ Name
       │           └─ `type`
       └─ Numeric
          └─ `1.`
    "#,
    false,
  );
}

#[test]
fn _0011() {
  let scope = scope!();
  let mut ctx_1 = ParsingContext::default();
  ctx_1.set_name("principal".into());
  ctx_1.set_name("rate".into());
  ctx_1.set_name("termMonths".into());
  scope.set_context("loan".into(), ctx_1);
  accept(
    &scope,
    StartExpression,
    r#"(loan.principal) + (loan.rate)"#,
    r#"
       Add
       ├─ Path
       │  ├─ Name
       │  │  └─ `loan`
       │  └─ Name
       │     └─ `principal`
       └─ Path
          ├─ Name
          │  └─ `loan`
          └─ Name
             └─ `rate`
    "#,
    false,
  );
}

#[test]
fn _0012() {
  let scope = scope!();
  let mut ctx_1 = ParsingContext::default();
  ctx_1.set_name("fromString".into());
  scope.set_context("Date".into(), ctx_1);
  accept(
    &scope,
    StartExpression,
    r#"Date.fromString.day"#,
    r#"
       Path
       ├─ Name
       │  └─ `Date`
       └─ Path
          ├─ Name
          │  └─ `fromString`
          └─ Name
             └─ `day`
    "#,
    false,
  );
}
