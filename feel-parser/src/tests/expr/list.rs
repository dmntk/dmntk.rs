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
use crate::lalr::TokenType::*;

#[test]
fn _0001() {
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
fn _0001_1() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        r#"[]"#,
        r#"
       List
       └─ (empty)
    "#,
        true,
    );
}

#[test]
fn _0002() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[[[]]]"#,
        r#"
       List
       └─ List
          └─ List
             └─ (empty)
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
        r#"[12.45]"#,
        r#"
       List
       └─ Numeric
          └─ `12.45`
    "#,
        false,
    );
}

#[test]
fn _0004() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"["family","home","job","car"]"#,
        r#"
       List
       ├─ String
       │  └─ `family`
       ├─ String
       │  └─ `home`
       ├─ String
       │  └─ `job`
       └─ String
          └─ `car`
    "#,
        false,
    );
}

#[test]
fn _0005() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[1.1,2.2]"#,
        r#"
       List
       ├─ Numeric
       │  └─ `1.1`
       └─ Numeric
          └─ `2.2`
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
        r#"["a","b","c"]"#,
        r#"
       List
       ├─ String
       │  └─ `a`
       ├─ String
       │  └─ `b`
       └─ String
          └─ `c`
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
        r#"[[1]]"#,
        r#"
       List
       └─ List
          └─ Numeric
             └─ `1.`
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
        r#"[[1],[2]]"#,
        r#"
       List
       ├─ List
       │  └─ Numeric
       │     └─ `1.`
       └─ List
          └─ Numeric
             └─ `2.`
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
        r#"[[1],[2],[3]]"#,
        r#"
       List
       ├─ List
       │  └─ Numeric
       │     └─ `1.`
       ├─ List
       │  └─ Numeric
       │     └─ `2.`
       └─ List
          └─ Numeric
             └─ `3.`
    "#,
        false,
    );
}

#[test]
fn _00010() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[[[1]]]"#,
        r#"
       List
       └─ List
          └─ List
             └─ Numeric
                └─ `1.`
    "#,
        false,
    );
}

#[test]
fn _00011() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[[[1],[2],[3]],[[4],[5]],[7,8,9]]"#,
        r#"
       List
       ├─ List
       │  ├─ List
       │  │  └─ Numeric
       │  │     └─ `1.`
       │  ├─ List
       │  │  └─ Numeric
       │  │     └─ `2.`
       │  └─ List
       │     └─ Numeric
       │        └─ `3.`
       ├─ List
       │  ├─ List
       │  │  └─ Numeric
       │  │     └─ `4.`
       │  └─ List
       │     └─ Numeric
       │        └─ `5.`
       └─ List
          ├─ Numeric
          │  └─ `7.`
          ├─ Numeric
          │  └─ `8.`
          └─ Numeric
             └─ `9.`
    "#,
        false,
    );
}
