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
    "[1,2,3][2]",
    r#"
       Filter
       ├─ List
       │  ├─ Numeric
       │  │  └─ `1.`
       │  ├─ Numeric
       │  │  └─ `2.`
       │  └─ Numeric
       │     └─ `3.`
       └─ Numeric
          └─ `2.`
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
    "[1,2,3][item >= 2]",
    r#"
       Filter
       ├─ List
       │  ├─ Numeric
       │  │  └─ `1.`
       │  ├─ Numeric
       │  │  └─ `2.`
       │  └─ Numeric
       │     └─ `3.`
       └─ Ge
          ├─ Name
          │  └─ `item`
          └─ Numeric
             └─ `2.`
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
    "EmployeeTable[name=LastName]",
    r#"
       Filter
       ├─ Name
       │  └─ `EmployeeTable`
       └─ Eq
          ├─ Name
          │  └─ `name`
          └─ Name
             └─ `LastName`
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
    "EmployeeTable[1].deptNum",
    r#"
       Path
       ├─ Filter
       │  ├─ Name
       │  │  └─ `EmployeeTable`
       │  └─ Numeric
       │     └─ `1.`
       └─ Name
          └─ `deptNum`
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
    r#"DeptTable[number=EmployeeTable[name=LastName].deptNum[1]].manager[1]"#,
    r#"
       Filter
       ├─ Path
       │  ├─ Filter
       │  │  ├─ Name
       │  │  │  └─ `DeptTable`
       │  │  └─ Eq
       │  │     ├─ Name
       │  │     │  └─ `number`
       │  │     └─ Filter
       │  │        ├─ Path
       │  │        │  ├─ Filter
       │  │        │  │  ├─ Name
       │  │        │  │  │  └─ `EmployeeTable`
       │  │        │  │  └─ Eq
       │  │        │  │     ├─ Name
       │  │        │  │     │  └─ `name`
       │  │        │  │     └─ Name
       │  │        │  │        └─ `LastName`
       │  │        │  └─ Name
       │  │        │     └─ `deptNum`
       │  │        └─ Numeric
       │  │           └─ `1.`
       │  └─ Name
       │     └─ `manager`
       └─ Numeric
          └─ `1.`
    "#,
    false,
  );
}

#[test]
fn _0006() {
  let scope = scope!();
  scope.set_name("?".into());
  scope.set_name("Lender Name".into());
  accept(
    &scope,
    StartExpression,
    r#"count(Ratings[Lender Name = ?.Lender Name and Customer Rating > 4]) > 0"#,
    r#"
       Gt
       ├─ FunctionInvocation
       │  ├─ Name
       │  │  └─ `count`
       │  └─ PositionalParameters
       │     └─ Filter
       │        ├─ Name
       │        │  └─ `Ratings`
       │        └─ And
       │           ├─ Eq
       │           │  ├─ Name
       │           │  │  └─ `Lender Name`
       │           │  └─ Path
       │           │     ├─ Name
       │           │     │  └─ `?`
       │           │     └─ Name
       │           │        └─ `Lender Name`
       │           └─ Gt
       │              ├─ Name
       │              │  └─ `Customer Rating`
       │              └─ Numeric
       │                 └─ `4.`
       └─ Numeric
          └─ `0.`
    "#,
    false,
  );
}
