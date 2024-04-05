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

//! Additional test for expressions found in compatibility tests.

use super::super::*;
use crate::context::ParsingContext;
use crate::lalr::TokenType::List;

#[test]
fn _0001() {
  let scope = scope!();
  let mut ctx = ParsingContext::default();
  ctx.set_name("principal".into());
  ctx.set_name("rate".into());
  ctx.set_name("termMonths".into());
  scope.set_context("loan".into(), ctx);
  accept(
    &scope,
    StartExpression,
    r#"(loan.principal*loan.rate/12)/(1-(1+loan.rate/12)**-loan.termMonths)"#,
    r#"
       Div
       ├─ Div
       │  ├─ Mul
       │  │  ├─ Path
       │  │  │  ├─ Name
       │  │  │  │  └─ `loan`
       │  │  │  └─ Name
       │  │  │     └─ `principal`
       │  │  └─ Path
       │  │     ├─ Name
       │  │     │  └─ `loan`
       │  │     └─ Name
       │  │        └─ `rate`
       │  └─ Numeric
       │     └─ `12.`
       └─ Sub
          ├─ Numeric
          │  └─ `1.`
          └─ Exp
             ├─ Add
             │  ├─ Numeric
             │  │  └─ `1.`
             │  └─ Div
             │     ├─ Path
             │     │  ├─ Name
             │     │  │  └─ `loan`
             │     │  └─ Name
             │     │     └─ `rate`
             │     └─ Numeric
             │        └─ `12.`
             └─ Neg
                └─ Path
                   ├─ Name
                   │  └─ `loan`
                   └─ Name
                      └─ `termMonths`
    "#,
    false,
  );
}

/// Tests how the parser behaves, when invalid starting point is given.
#[test]
fn _0002() {
  let scope = scope!();
  assert!(Parser::new(&scope, List, "[1,2,3]", false).parse().is_err());
}
