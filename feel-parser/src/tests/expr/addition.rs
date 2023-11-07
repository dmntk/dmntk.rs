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
use crate::parser::Parser;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "1+2",
        r#"
       Add
       ├─ Numeric
       │  └─ `1.`
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
        "1 + 2",
        r#"
       Add
       ├─ Numeric
       │  └─ `1.`
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
        " 5 +2 +1",
        r#"
       Add
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `5.`
       │  └─ Numeric
       │     └─ `2.`
       └─ Numeric
          └─ `1.`
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
        "2+20+200",
        r#"
       Add
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `2.`
       │  └─ Numeric
       │     └─ `20.`
       └─ Numeric
          └─ `200.`
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
        "( 1 + 2 ) + ( 3 + 4 )",
        r#"
       Add
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `1.`
       │  └─ Numeric
       │     └─ `2.`
       └─ Add
          ├─ Numeric
          │  └─ `3.`
          └─ Numeric
             └─ `4.`
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
        "( ( ( 1 + 2 ) ) )",
        r#"
       Add
       ├─ Numeric
       │  └─ `1.`
       └─ Numeric
          └─ `2.`
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
        "(1+2)*(3+4)",
        r#"
       Mul
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `1.`
       │  └─ Numeric
       │     └─ `2.`
       └─ Add
          ├─ Numeric
          │  └─ `3.`
          └─ Numeric
             └─ `4.`
    "#,
        false,
    );
}

#[test]
fn _0008() {
    let scope = scope!();
    assert_eq!(
        "<ParserError> syntax error: +1",
        Parser::new(&scope, StartExpression, "+1", false)
            .parse()
            .err()
            .unwrap()
            .to_string()
            .as_str()
    );
}
