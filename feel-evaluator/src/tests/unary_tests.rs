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
use dmntk_feel::scope;

#[test]
fn test_0001() {
  valid_unary_tests(false, &scope!(), "-");
}

#[test]
fn test_0002() {
  valid_unary_tests(false, &scope!(), "1");
}

#[test]
fn test_0003() {
  valid_unary_tests(false, &scope!(), "1,2");
}

#[test]
fn test_0004() {
  valid_unary_tests(false, &scope!(), "1,2,3");
}

#[test]
fn test_0005() {
  valid_unary_tests(false, &scope!(), "[]");
}

#[test]
fn test_0006() {
  valid_unary_tests(false, &scope!(), "[1]");
}

#[test]
fn test_0007() {
  valid_unary_tests(false, &scope!(), "[1,2]");
}

#[test]
fn test_0008() {
  valid_unary_tests(false, &scope!(), "[1,2,3]");
}

#[test]
fn test_0009() {
  valid_unary_tests(false, &scope!(), "[1..2]");
}

#[test]
fn test_0010() {
  valid_unary_tests(false, &scope!(), "not(1,2,3)");
}
