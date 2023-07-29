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
use crate::lalr::TokenType::StartExpression;

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "1.364 = 2.9483",
    r#"
       Eq
       ├─ Numeric
       │  └─ `1.364`
       └─ Numeric
          └─ `2.9483`
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
    "5.678 != 5.679",
    r#"
       Nq
       ├─ Numeric
       │  └─ `5.678`
       └─ Numeric
          └─ `5.679`
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
    "-23.45 < -5.28374658493",
    r#"
       Lt
       ├─ Neg
       │  └─ Numeric
       │     └─ `23.45`
       └─ Neg
          └─ Numeric
             └─ `5.28374658493`
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
    "-23.45 <= -5.28374658493",
    r#"
       Le
       ├─ Neg
       │  └─ Numeric
       │     └─ `23.45`
       └─ Neg
          └─ Numeric
             └─ `5.28374658493`
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
    "5 > 3",
    r#"
       Gt
       ├─ Numeric
       │  └─ `5.`
       └─ Numeric
          └─ `3.`
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
    "5 >= 3",
    r#"
       Ge
       ├─ Numeric
       │  └─ `5.`
       └─ Numeric
          └─ `3.`
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
    "5 between 1 and 10",
    r#"
       Between
       ├─ Numeric
       │  └─ `5.`
       ├─ Numeric
       │  └─ `1.`
       └─ Numeric
          └─ `10.`
    "#,
    false,
  );
}

#[test]
fn _0008() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "5 in 10",
    r#"
       In
       ├─ Numeric
       │  └─ `5.`
       └─ Numeric
          └─ `10.`
    "#,
    false,
  );
}

#[test]
fn _0009() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "5 in <10",
    r#"
       In
       ├─ Numeric
       │  └─ `5.`
       └─ UnaryLt
          └─ Numeric
             └─ `10.`
    "#,
    false,
  );
}

#[test]
fn _0010() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "5 in <=10",
    r#"
       In
       ├─ Numeric
       │  └─ `5.`
       └─ UnaryLe
          └─ Numeric
             └─ `10.`
    "#,
    false,
  );
}

#[test]
fn _0011() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "18 in >8",
    r#"
       In
       ├─ Numeric
       │  └─ `18.`
       └─ UnaryGt
          └─ Numeric
             └─ `8.`
    "#,
    false,
  );
}

#[test]
fn _0012() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "18 in >=8",
    r#"
       In
       ├─ Numeric
       │  └─ `18.`
       └─ UnaryGe
          └─ Numeric
             └─ `8.`
    "#,
    false,
  );
}

#[test]
fn _0013() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "18 in [1..100]",
    r#"
       In
       ├─ Numeric
       │  └─ `18.`
       └─ Range
          ├─ IntervalStart (closed)
          │  └─ Numeric
          │     └─ `1.`
          └─ IntervalEnd (closed)
             └─ Numeric
                └─ `100.`
    "#,
    false,
  );
}

#[test]
fn _0014() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "18 in (1..100)",
    r#"
       In
       ├─ Numeric
       │  └─ `18.`
       └─ Range
          ├─ IntervalStart (opened)
          │  └─ Numeric
          │     └─ `1.`
          └─ IntervalEnd (opened)
             └─ Numeric
                └─ `100.`
    "#,
    false,
  );
}

#[test]
fn _0015() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "5 in (1,2,3,4,5,6,7)",
    r#"
       In
       ├─ Numeric
       │  └─ `5.`
       └─ ExpressionList
          ├─ Numeric
          │  └─ `1.`
          ├─ Numeric
          │  └─ `2.`
          ├─ Numeric
          │  └─ `3.`
          ├─ Numeric
          │  └─ `4.`
          ├─ Numeric
          │  └─ `5.`
          ├─ Numeric
          │  └─ `6.`
          └─ Numeric
             └─ `7.`
    "#,
    false,
  );
}
