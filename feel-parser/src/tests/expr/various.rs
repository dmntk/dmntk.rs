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
  let mut ctx_1 = ParsingContext::default();
  ctx_1.set_name("tenor".into());
  ctx_1.set_name("rate".into());
  scope.set_context("offer".into(), ctx_1);
  let mut ctx_2 = ParsingContext::default();
  ctx_2.set_name("tenor".into());
  ctx_2.set_name("rate".into());
  scope.set_context("bid".into(), ctx_2);
  accept(
    &scope,
    StartExpression,
    r#"for i in 1..6 return ((log(1 + (offer[i].rate / 100) * (offer[i].tenor / 365))) / (offer[i].tenor / 365) + (log(1 + (bid[i].rate / 100) * (bid[i].tenor / 365))) / (bid[i].tenor / 365)) / 2"#,
    r#"
       For
       ├─ IterationContexts
       │  └─ IterationContextRange
       │     ├─ Name
       │     │  └─ `i`
       │     ├─ Numeric
       │     │  └─ `1.`
       │     └─ Numeric
       │        └─ `6.`
       └─ EvaluatedExpression
          └─ Div
             ├─ Add
             │  ├─ Div
             │  │  ├─ FunctionInvocation
             │  │  │  ├─ Name
             │  │  │  │  └─ `log`
             │  │  │  └─ PositionalParameters
             │  │  │     └─ Add
             │  │  │        ├─ Numeric
             │  │  │        │  └─ `1.`
             │  │  │        └─ Mul
             │  │  │           ├─ Div
             │  │  │           │  ├─ Path
             │  │  │           │  │  ├─ Filter
             │  │  │           │  │  │  ├─ Name
             │  │  │           │  │  │  │  └─ `offer`
             │  │  │           │  │  │  └─ Name
             │  │  │           │  │  │     └─ `i`
             │  │  │           │  │  └─ Name
             │  │  │           │  │     └─ `rate`
             │  │  │           │  └─ Numeric
             │  │  │           │     └─ `100.`
             │  │  │           └─ Div
             │  │  │              ├─ Path
             │  │  │              │  ├─ Filter
             │  │  │              │  │  ├─ Name
             │  │  │              │  │  │  └─ `offer`
             │  │  │              │  │  └─ Name
             │  │  │              │  │     └─ `i`
             │  │  │              │  └─ Name
             │  │  │              │     └─ `tenor`
             │  │  │              └─ Numeric
             │  │  │                 └─ `365.`
             │  │  └─ Div
             │  │     ├─ Path
             │  │     │  ├─ Filter
             │  │     │  │  ├─ Name
             │  │     │  │  │  └─ `offer`
             │  │     │  │  └─ Name
             │  │     │  │     └─ `i`
             │  │     │  └─ Name
             │  │     │     └─ `tenor`
             │  │     └─ Numeric
             │  │        └─ `365.`
             │  └─ Div
             │     ├─ FunctionInvocation
             │     │  ├─ Name
             │     │  │  └─ `log`
             │     │  └─ PositionalParameters
             │     │     └─ Add
             │     │        ├─ Numeric
             │     │        │  └─ `1.`
             │     │        └─ Mul
             │     │           ├─ Div
             │     │           │  ├─ Path
             │     │           │  │  ├─ Filter
             │     │           │  │  │  ├─ Name
             │     │           │  │  │  │  └─ `bid`
             │     │           │  │  │  └─ Name
             │     │           │  │  │     └─ `i`
             │     │           │  │  └─ Name
             │     │           │  │     └─ `rate`
             │     │           │  └─ Numeric
             │     │           │     └─ `100.`
             │     │           └─ Div
             │     │              ├─ Path
             │     │              │  ├─ Filter
             │     │              │  │  ├─ Name
             │     │              │  │  │  └─ `bid`
             │     │              │  │  └─ Name
             │     │              │  │     └─ `i`
             │     │              │  └─ Name
             │     │              │     └─ `tenor`
             │     │              └─ Numeric
             │     │                 └─ `365.`
             │     └─ Div
             │        ├─ Path
             │        │  ├─ Filter
             │        │  │  ├─ Name
             │        │  │  │  └─ `bid`
             │        │  │  └─ Name
             │        │  │     └─ `i`
             │        │  └─ Name
             │        │     └─ `tenor`
             │        └─ Numeric
             │           └─ `365.`
             └─ Numeric
                └─ `2.`
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  scope.set_name("Days".into());
  let mut ctx_1 = ParsingContext::default();

  let mut ctx_2 = ParsingContext::default();
  ctx_2.set_name("Tenor".into());
  ctx_2.set_name("Rate".into());

  ctx_1.set_context("Min".into(), ctx_2);

  let mut ctx_3 = ParsingContext::default();
  ctx_3.set_name("Tenor".into());
  ctx_3.set_name("Rate".into());

  ctx_1.set_context("Max".into(), ctx_3);

  scope.set_context("Bounds".into(), ctx_1);

  let input = r#"(((Bounds.Max.Rate - Bounds.Min.Rate) * (Days - Bounds.Min.Tenor)) / (Bounds.Max.Tenor - Bounds.Min.Tenor)) + Bounds.Min.Rate"#;
  accept(
    &scope,
    StartExpression,
    input,
    r#"
       Add
       ├─ Div
       │  ├─ Mul
       │  │  ├─ Sub
       │  │  │  ├─ Path
       │  │  │  │  ├─ Name
       │  │  │  │  │  └─ `Bounds`
       │  │  │  │  └─ Path
       │  │  │  │     ├─ Name
       │  │  │  │     │  └─ `Max`
       │  │  │  │     └─ Name
       │  │  │  │        └─ `Rate`
       │  │  │  └─ Path
       │  │  │     ├─ Name
       │  │  │     │  └─ `Bounds`
       │  │  │     └─ Path
       │  │  │        ├─ Name
       │  │  │        │  └─ `Min`
       │  │  │        └─ Name
       │  │  │           └─ `Rate`
       │  │  └─ Sub
       │  │     ├─ Name
       │  │     │  └─ `Days`
       │  │     └─ Path
       │  │        ├─ Name
       │  │        │  └─ `Bounds`
       │  │        └─ Path
       │  │           ├─ Name
       │  │           │  └─ `Min`
       │  │           └─ Name
       │  │              └─ `Tenor`
       │  └─ Sub
       │     ├─ Path
       │     │  ├─ Name
       │     │  │  └─ `Bounds`
       │     │  └─ Path
       │     │     ├─ Name
       │     │     │  └─ `Max`
       │     │     └─ Name
       │     │        └─ `Tenor`
       │     └─ Path
       │        ├─ Name
       │        │  └─ `Bounds`
       │        └─ Path
       │           ├─ Name
       │           │  └─ `Min`
       │           └─ Name
       │              └─ `Tenor`
       └─ Path
          ├─ Name
          │  └─ `Bounds`
          └─ Path
             ├─ Name
             │  └─ `Min`
             └─ Name
                └─ `Rate`
    "#,
    false,
  );
}

#[test]
fn _0003() {
  let input = r#"Flights[ From = Original Flight.From and To = Original Flight.To and Departure > Original Flight.Departure and Status = "scheduled" and has capacity(item, Reassigned Passengers List)][1]"#;
  // this test should fail without properly set scope
  let node = Parser::new(&scope!(), StartExpression, input, false).parse();
  assert!(node.is_err());
}
