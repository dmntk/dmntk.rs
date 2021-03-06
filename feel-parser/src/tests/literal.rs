/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
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
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
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

use super::accept;
use crate::lalr::TokenType::StartTextualExpression;
use dmntk_feel::{scope, Scope};

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "null",
    r#"
       Null
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#" "text" "#,
    r#"
       String
       └─ `text`
    "#,
    false,
  );
}

#[test]
fn _0003() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#" "line \n line" "#,
    r#"
       String
       └─ `line 
       line`
    "#,
    false,
  );
}

#[test]
fn _0004() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#" "before\u002Bafter" "#,
    r#"
       String
       └─ `before+after`
    "#,
    false,
  );
}

#[test]
fn _0005() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#" "before\U00002Bafter" "#,
    r#"
       String
       └─ `before+after`
    "#,
    false,
  );
}

#[test]
fn _0006() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#" "line \n line \t line \r line" "#,
    "\n       String\n       └─ `line \n       line \t line \r line`\n    ",
    false,
  );
}

#[test]
fn _0007() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#" "line \' line \" line \\ line" "#,
    r#"
       String
       └─ `line ' line " line \ line`
    "#,
    false,
  );
}

#[test]
fn _0008() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#" "a \u07EF b" "#,
    r#"
       String
       └─ `a ߯ b`
    "#,
    false,
  );
}

#[test]
fn _0009() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#" "a \u0801 b" "#,
    r#"
       String
       └─ `a ࠁ b`
    "#,
    false,
  );
}

#[test]
fn _0010() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#" "a \U010001 b" "#,
    r#"
       String
       └─ `a 𐀁 b`
    "#,
    false,
  );
}

#[test]
fn _0011() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "\"a \\U010437 b\"",
    r#"
       String
       └─ `a 𐐷 b`
    "#,
    false,
  );
}

#[test]
fn _0012() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#" "a \uD801\uDC12 b" "#,
    r#"
       String
       └─ `a 𐐒 b`
    "#,
    false,
  );
}

#[test]
fn _0013() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#" "a \u0056 \u00A9 b" "#,
    r#"
       String
       └─ `a V © b`
    "#,
    false,
  );
}

#[test]
fn _0014() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#""earth""#,
    r#"
       String
       └─ `earth`
    "#,
    false,
  );
}

#[test]
#[should_panic]
fn _0015() {
  let scope = scope!();
  accept(&scope, StartTextualExpression, r#" "\u000G" "#, r#"  String `a V © b` "#, false);
}

#[test]
fn _0016() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "true",
    r#"
       Boolean
       └─ `true`
    "#,
    false,
  );
}

#[test]
fn _0017() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "false",
    r#"
       Boolean
       └─ `false`
    "#,
    false,
  );
}

#[test]
fn _0018() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#"@"2021-09-23""#,
    r#"
       At
       └─ `2021-09-23`
    "#,
    false,
  );
}

#[test]
fn _0019() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#"date("2021-09-23")"#,
    r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date`
       └─ PositionalParameters
          └─ String
             └─ `2021-09-23`
    "#,
    false,
  );
}

#[test]
fn _0020() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#"time("10:23:45")"#,
    r#"
       FunctionInvocation
       ├─ Name
       │  └─ `time`
       └─ PositionalParameters
          └─ String
             └─ `10:23:45`
    "#,
    false,
  );
}

#[test]
fn _0021() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#"date and time("2021-09-23 10:23:45")"#,
    r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date and time`
       └─ PositionalParameters
          └─ String
             └─ `2021-09-23 10:23:45`
    "#,
    false,
  );
}

#[test]
fn _0022() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#"duration("P1Y")"#,
    r#"
       FunctionInvocation
       ├─ Name
       │  └─ `duration`
       └─ PositionalParameters
          └─ String
             └─ `P1Y`
    "#,
    false,
  );
}
