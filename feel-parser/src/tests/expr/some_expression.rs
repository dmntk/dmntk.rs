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

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"some n in [1,2,3] satisfies n > 1.5"#,
    r#"
       Some
       ├─ QuantifiedContexts
       │  └─ QuantifiedContext
       │     ├─ Name
       │     │  └─ `n`
       │     └─ List
       │        ├─ Numeric
       │        │  └─ `1.`
       │        ├─ Numeric
       │        │  └─ `2.`
       │        └─ Numeric
       │           └─ `3.`
       └─ Satisfies
          └─ Gt
             ├─ Name
             │  └─ `n`
             └─ Numeric
                └─ `1.5`
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
    r#"some n in [1,2,3] satisfies n + 1 > 1.5"#,
    r#"
       Some
       ├─ QuantifiedContexts
       │  └─ QuantifiedContext
       │     ├─ Name
       │     │  └─ `n`
       │     └─ List
       │        ├─ Numeric
       │        │  └─ `1.`
       │        ├─ Numeric
       │        │  └─ `2.`
       │        └─ Numeric
       │           └─ `3.`
       └─ Satisfies
          └─ Gt
             ├─ Add
             │  ├─ Name
             │  │  └─ `n`
             │  └─ Numeric
             │     └─ `1.`
             └─ Numeric
                └─ `1.5`
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
    r#"some n in [1,2,3], m in <= 100 satisfies n > 1.5 * m"#,
    r#"
       Some
       ├─ QuantifiedContexts
       │  ├─ QuantifiedContext
       │  │  ├─ Name
       │  │  │  └─ `n`
       │  │  └─ List
       │  │     ├─ Numeric
       │  │     │  └─ `1.`
       │  │     ├─ Numeric
       │  │     │  └─ `2.`
       │  │     └─ Numeric
       │  │        └─ `3.`
       │  └─ QuantifiedContext
       │     ├─ Name
       │     │  └─ `m`
       │     └─ UnaryLe
       │        └─ Numeric
       │           └─ `100.`
       └─ Satisfies
          └─ Gt
             ├─ Name
             │  └─ `n`
             └─ Mul
                ├─ Numeric
                │  └─ `1.5`
                └─ Name
                   └─ `m`
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
    r#"some n in [1..3] satisfies n > 1.5"#,
    r#"
       Some
       ├─ QuantifiedContexts
       │  └─ QuantifiedContext
       │     ├─ Name
       │     │  └─ `n`
       │     └─ Range
       │        ├─ IntervalStart (closed)
       │        │  └─ Numeric
       │        │     └─ `1.`
       │        └─ IntervalEnd (closed)
       │           └─ Numeric
       │              └─ `3.`
       └─ Satisfies
          └─ Gt
             ├─ Name
             │  └─ `n`
             └─ Numeric
                └─ `1.5`
    "#,
    false,
  );
}
