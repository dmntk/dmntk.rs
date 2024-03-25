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
use dmntk_feel::Name;

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"<2"#,
    r#"
       UnaryLt
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
    r#" <= 12.465"#,
    r#"
       UnaryLe
       └─ Numeric
          └─ `12.465`
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
    r#" > 50"#,
    r#"
       UnaryGt
       └─ Numeric
          └─ `50.`
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
    r#" >= time("10:23")"#,
    r#"
       UnaryGe
       └─ FunctionInvocation
          ├─ Name
          │  └─ `time`
          └─ PositionalParameters
             └─ String
                └─ `10:23`
    "#,
    false,
  );
}

#[test]
fn _0005() {
  let scope = scope!();
  let mut ctx = ParsingContext::default();
  let name_power = Name::from("power");
  ctx.set_name(name_power);
  scope.set_context("engine".into(), ctx);
  accept(
    &scope,
    StartExpression,
    r#" >= engine.power"#,
    r#"
       UnaryGe
       └─ QualifiedName
          ├─ Name
          │  └─ `engine`
          └─ Name
             └─ `power`
    "#,
    false,
  );
}

#[test]
fn _0006() {
  let scope = scope!();
  scope.set_name("engine".into());
  scope.set_name("power".into());
  accept(
    &scope,
    StartExpression,
    r#" >= engine.power"#,
    r#"
       UnaryGe
       └─ QualifiedName
          ├─ Name
          │  └─ `engine`
          └─ Name
             └─ `power`
    "#,
    false,
  );
}

#[test]
#[should_panic]
fn _0007() {
  let scope = scope!();
  accept(&scope, StartExpression, r#" < null"#, r#""#, false);
}
