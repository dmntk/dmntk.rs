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

#![feature(test)]

extern crate test;

use dmntk_feel_number::FeelNumber;
use test::Bencher;

#[bench]
fn bench_add_001(b: &mut Bencher) {
  let x = FeelNumber::new(1, 1);
  let y = FeelNumber::new(3, 1);
  b.iter(|| x + y)
}

#[bench]
fn bench_add_002(b: &mut Bencher) {
  let x = FeelNumber::new(12345, 2);
  let y = FeelNumber::new(3847847, 7);
  b.iter(|| x + y)
}

#[bench]
fn bench_add_003(b: &mut Bencher) {
  let x = FeelNumber::new(123, 2);
  let y = FeelNumber::new(77, 1);
  b.iter(|| x + y)
}

#[bench]
fn bench_add_004(b: &mut Bencher) {
  let mut x = FeelNumber::new(1, 1);
  let y = FeelNumber::new(3, 1);
  b.iter(|| x += y)
}

#[bench]
fn bench_add_005(b: &mut Bencher) {
  let mut x = FeelNumber::new(12345, 2);
  let y = FeelNumber::new(3847847, 7);
  b.iter(|| x += y)
}

#[bench]
fn bench_add_006(b: &mut Bencher) {
  let mut x = FeelNumber::new(123, 2);
  let y = FeelNumber::new(77, 1);
  b.iter(|| x += y)
}
