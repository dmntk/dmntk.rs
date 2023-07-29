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
use crate::lalr::TokenType::{StartContext, StartExpression};

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"  1  // eol comment
         + 1"#,
    r#"
       Add
       ├─ Numeric
       │  └─ `1.`
       └─ Numeric
          └─ `1.`
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
    r#" 1
          /*
          some intro waffle
          */
          + 1"#,
    r#"
       Add
       ├─ Numeric
       │  └─ `1.`
       └─ Numeric
          └─ `1.`
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
    r#"1 + /* 1 + */ 1"#,
    r#"
       Add
       ├─ Numeric
       │  └─ `1.`
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
    r#" 1
          /*
          some intro waffle
          */
          + 1 // and stuff
          + 2"#,
    r#"
       Add
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `1.`
       │  └─ Numeric
       │     └─ `1.`
       └─ Numeric
          └─ `2.`
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
    r#"
            // Some multi-line comment
            // composed from
            // multiple single-line
            // comments
            1 + 2 
    "#,
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
fn _0006() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"
            /*
             * Some multi-line comment
             * composed from...
             */
             
            /*
             *
             * ...multiple multi-line
             * comments
             */ 
             
            5 * 8 
    "#,
    r#"
       Mul
       ├─ Numeric
       │  └─ `5.`
       └─ Numeric
          └─ `8.`
    "#,
    false,
  );
}

#[test]
fn _0007() {
  let scope = scope!();
  accept(
    &scope,
    StartContext,
    r#"
      /// Maybe this comment may be used
      /// for some documentation, like in Rust?
      
      //! Or for some global documentation too, like in Rust?
      {
        /// Maybe this comment may be used for some documentation, like in Rust?
        
        /*
         * We will see.
         */
        A: /* This is cool :-), not like in JSON :-( */ 15 // Just set fiveteen.
      }
    "#,
    r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `A`
          └─ Numeric
             └─ `15.`
    "#,
    false,
  );
}
