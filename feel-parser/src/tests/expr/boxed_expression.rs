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
use crate::lalr::TokenType::StartBoxedExpression;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"function () "the body" "#,
        r#"
       FunctionDefinition
       ├─ FormalParameters
       │  └─ (empty)
       └─ FunctionBody
          └─ String
             └─ `the body`
    "#,
        false,
    );
}

#[test]
fn _0002() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"function () 1.414926 "#,
        r#"
       FunctionDefinition
       ├─ FormalParameters
       │  └─ (empty)
       └─ FunctionBody
          └─ Numeric
             └─ `1.414926`
    "#,
        false,
    );
}

#[test]
fn _0003() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"function () external { java: {class: "java.lang.Math", method signature: "min"} }"#,
        r#"
       FunctionDefinition
       ├─ FormalParameters
       │  └─ (empty)
       └─ FunctionBody (external)
          └─ Context
             └─ ContextEntry
                ├─ ContextEntryKey
                │  └─ `java`
                └─ Context
                   ├─ ContextEntry
                   │  ├─ ContextEntryKey
                   │  │  └─ `class`
                   │  └─ String
                   │     └─ `java.lang.Math`
                   └─ ContextEntry
                      ├─ ContextEntryKey
                      │  └─ `method signature`
                      └─ String
                         └─ `min`
    "#,
        false,
    );
}

#[test]
#[should_panic]
fn _0004() {
    let scope = scope!();
    accept(&scope, StartBoxedExpression, r#""#, r#""#, false);
}

#[test]
fn _0005() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[]"#,
        r#"
       List
       └─ (empty)
    "#,
        false,
    );
}

#[test]
fn _0006() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[1]"#,
        r#"
       List
       └─ Numeric
          └─ `1.`
    "#,
        false,
    );
}

#[test]
fn _0007() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#" [ 1 , 2 , 3 ] "#,
        r#"
       List
       ├─ Numeric
       │  └─ `1.`
       ├─ Numeric
       │  └─ `2.`
       └─ Numeric
          └─ `3.`
    "#,
        false,
    );
}

#[test]
fn _0008() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#" {} "#,
        r#"
       Context
       └─ (empty)
    "#,
        false,
    );
}

#[test]
fn _0009() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#" { age: 50 } "#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `age`
          └─ Numeric
             └─ `50.`
    "#,
        false,
    );
}

#[test]
fn _0010() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#" { name: "John", addres: { street: "Bourbon Street" , house no: 15} , married: false } "#,
        r#"
       Context
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `name`
       │  └─ String
       │     └─ `John`
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `addres`
       │  └─ Context
       │     ├─ ContextEntry
       │     │  ├─ ContextEntryKey
       │     │  │  └─ `street`
       │     │  └─ String
       │     │     └─ `Bourbon Street`
       │     └─ ContextEntry
       │        ├─ ContextEntryKey
       │        │  └─ `house no`
       │        └─ Numeric
       │           └─ `15.`
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `married`
          └─ Boolean
             └─ `false`
    "#,
        false,
    );
}
