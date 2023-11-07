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
use dmntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"list contains([1,2,3,4],2)"#, true);
}

#[test]
fn _0002() {
    te_bool(
        false,
        &scope!(),
        r#"list contains([1,2,null,4],null)"#,
        true,
    );
}

#[test]
fn _0003() {
    te_bool(false, &scope!(), r#"list contains([1,2,3,4],8)"#, false);
}

#[test]
fn _0004() {
    te_bool(
        false,
        &scope!(),
        r#"list contains(list: [1,2,3,4],match: 2)"#,
        true,
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"list contains()"#,
        r#"expected 2 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"list contains([1,2,3])"#,
        r#"expected 2 parameters, actual number of parameters is 1"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"list contains([1,2,3],2,6)"#,
        r#"expected 2 parameters, actual number of parameters is 3"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"list contains(10,2)"#,
        r#"[core::list contains] invalid argument type, expected list, actual type is number"#,
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"list contains(l:[10,11,12],match:11)"#,
        r#"parameter 'list' not found"#,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"list contains(list:[10,11,12],m:11)"#,
        r#"parameter 'match' not found"#,
    );
}
