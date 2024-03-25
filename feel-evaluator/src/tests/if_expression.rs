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

use super::*;

#[test]
fn _0001() {
  let scope = &te_scope(r#"{N:9}"#);
  te_number(false, scope, r#"if N < 10 then 1 else 2"#, 1, 0);
}

#[test]
fn _0002() {
  let scope = &te_scope(r#"{N:10}"#);
  te_number(false, scope, r#"if N < 10 then 1 else 2"#, 2, 0);
}

#[test]
fn _0003() {
  let scope = &te_scope(r#"{aDate: @"2017-01-02", aString: "Hello World"}"#);
  te_string(
    false,
    scope,
    r#"if aDate > date("2017-01-01") then substring before(aString, " ") else substring after(aString, " ")"#,
    r#"Hello"#,
  );
}

#[test]
fn _0004() {
  let scope = &te_scope(r#"{aDate: @"2017-01-01", aString: "Hello World"}"#);
  te_string(
    false,
    scope,
    r#"if aDate > date("2017-01-01") then substring before(aString, " ") else substring after(aString, " ")"#,
    r#"World"#,
  );
}

#[test]
fn _0005() {
  let scope = &te_scope(r#"{N:9}"#);
  te_null(false, scope, r#"if N then 1 else 2"#, r#"condition in 'if' expression is not a boolean value"#);
}
