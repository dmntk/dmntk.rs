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
    r#"[1..10]"#,
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
    r#"date("2012-12-25") in [date("2012-01-01")..date("2021-12-31")]"#,
    r#"
       In
       ├─ FunctionInvocation
       │  ├─ Name
       │  │  └─ `date`
       │  └─ PositionalParameters
       │     └─ String
       │        └─ `2012-12-25`
       └─ Range
          ├─ IntervalStart (closed)
          │  └─ FunctionInvocation
          │     ├─ Name
          │     │  └─ `date`
          │     └─ PositionalParameters
          │        └─ String
          │           └─ `2012-01-01`
          └─ IntervalEnd (closed)
             └─ FunctionInvocation
                ├─ Name
                │  └─ `date`
                └─ PositionalParameters
                   └─ String
                      └─ `2021-12-31`
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
    r#"(<=10) = [1..10]"#,
    r#"
       Eq
       ├─ UnaryLe
       │  └─ Numeric
       │     └─ `10.`
       └─ Range
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
