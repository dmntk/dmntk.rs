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

#![feature(test)]

extern crate test;

use dmntk_feel::Name;
use test::Bencher;

#[bench]
fn feel_name_equal_0001(b: &mut Bencher) {
  let name: Name = "a".into();
  b.iter(|| (name == name));
}

#[bench]
fn feel_name_equal_0002(b: &mut Bencher) {
  let name: Name = vec!["a", "b"].into();
  b.iter(|| name == name);
}

#[bench]
fn feel_name_equal_0003(b: &mut Bencher) {
  let name: Name = vec!["a", "b", "c"].into();
  b.iter(|| name == name);
}

#[bench]
fn feel_name_equal_0004(b: &mut Bencher) {
  let name: Name = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"].into();
  b.iter(|| name == name);
}

#[bench]
fn feel_name_equal_0005(b: &mut Bencher) {
  let name_a: Name = vec!["a", "b"].into();
  let name_b: Name = vec!["a b"].into();
  b.iter(|| name_a == name_b);
}

#[bench]
fn feel_name_equal_0006(b: &mut Bencher) {
  let name_a: Name = vec!["a", "b", "c"].into();
  let name_b: Name = vec!["a b c"].into();
  b.iter(|| name_a == name_b);
}

#[bench]
fn feel_name_equal_0007(b: &mut Bencher) {
  let name_a: Name = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"].into();
  let name_b: Name = vec!["a b c d e f g h i j"].into();
  b.iter(|| name_a == name_b);
}
