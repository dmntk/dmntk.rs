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
use crate::context::ParsingContext;
use crate::lalr::TokenType::*;

#[test]
fn _0001() {
  let scope = scope!();
  scope.set_name("name".into());
  accept(
    &scope,
    StartExpression,
    r#"{greeting: function (name) "Hello " + name + "!" }"#,
    r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `greeting`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `name`
             │     └─ FeelType
             │        └─ Any
             └─ FunctionBody
                └─ Add
                   ├─ Add
                   │  ├─ String
                   │  │  └─ `Hello `
                   │  └─ Name
                   │     └─ `name`
                   └─ String
                      └─ `!`
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  scope.set_name("suffix".into());
  scope.set_name("other".into());
  scope.set_name("name".into());
  accept(
    &scope,
    StartExpression,
    r#"{greeting: function (name) "Hello " + name + "! (" + suffix + ")" }"#,
    r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `greeting`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `name`
             │     └─ FeelType
             │        └─ Any
             └─ FunctionBody
                └─ Add
                   ├─ Add
                   │  ├─ Add
                   │  │  ├─ Add
                   │  │  │  ├─ String
                   │  │  │  │  └─ `Hello `
                   │  │  │  └─ Name
                   │  │  │     └─ `name`
                   │  │  └─ String
                   │  │     └─ `! (`
                   │  └─ Name
                   │     └─ `suffix`
                   └─ String
                      └─ `)`
    "#,
    false,
  );
}

#[test]
fn _0003() {
  // prepare the context with names
  let mut ctx_inner = ParsingContext::default();
  ctx_inner.set_name("Surname".into());
  ctx_inner.set_name("City".into());
  ctx_inner.set_name("Street".into());
  ctx_inner.set_name(vec!["Marital", "status"].into());
  let mut ctx_outer = ParsingContext::default();
  ctx_outer.set_context("Person".into(), ctx_inner);
  let mut ctx = ParsingContext::default();
  ctx.set_context(vec!["Customer", "data"].into(), ctx_outer);
  // prepare scope
  let scope = scope!();
  scope.set_name("x".into());
  scope.set_name("y".into());
  scope.set_name(vec!["Customer", "data"].into());
  scope.push(ctx);
  assert_eq!(
    "[{Customer data: <v>, x: <v>, y: <v>}, {Customer data: {Person: {City: <v>, Marital status: <v>, Street: <v>, Surname: <v>}}}]",
    scope.to_string()
  );
  accept(
    &scope,
    StartExpression,
    r#"{greeting: function (prefix) prefix + ", hello " + Customer    data.Person.Surname + " (" + Customer                     data.Person.City + ")" + x }"#,
    r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `greeting`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `prefix`
             │     └─ FeelType
             │        └─ Any
             └─ FunctionBody
                └─ Add
                   ├─ Add
                   │  ├─ Add
                   │  │  ├─ Add
                   │  │  │  ├─ Add
                   │  │  │  │  ├─ Add
                   │  │  │  │  │  ├─ Name
                   │  │  │  │  │  │  └─ `prefix`
                   │  │  │  │  │  └─ String
                   │  │  │  │  │     └─ `, hello `
                   │  │  │  │  └─ Path
                   │  │  │  │     ├─ Name
                   │  │  │  │     │  └─ `Customer data`
                   │  │  │  │     └─ Path
                   │  │  │  │        ├─ Name
                   │  │  │  │        │  └─ `Person`
                   │  │  │  │        └─ Name
                   │  │  │  │           └─ `Surname`
                   │  │  │  └─ String
                   │  │  │     └─ ` (`
                   │  │  └─ Path
                   │  │     ├─ Name
                   │  │     │  └─ `Customer data`
                   │  │     └─ Path
                   │  │        ├─ Name
                   │  │        │  └─ `Person`
                   │  │        └─ Name
                   │  │           └─ `City`
                   │  └─ String
                   │     └─ `)`
                   └─ Name
                      └─ `x`
    "#,
    false,
  );
  //assert_eq!("[prefix, Customer data.Person.Surname, Customer data.Person.City, x]", closure.to_string());
}
