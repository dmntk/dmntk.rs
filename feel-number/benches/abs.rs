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

use dmntk_feel_number::FeelNumber;
use test::Bencher;

#[bench]
fn bench_abs_001(b: &mut Bencher) {
  let n = FeelNumber::new(0, 0);
  b.iter(|| n.abs())
}

#[bench]
fn bench_abs_002(b: &mut Bencher) {
  let n = FeelNumber::new(-0, 0);
  b.iter(|| n.abs())
}

#[bench]
fn bench_abs_003(b: &mut Bencher) {
  let n = FeelNumber::new(1, 0);
  b.iter(|| n.abs())
}

#[bench]
fn bench_abs_004(b: &mut Bencher) {
  let n = FeelNumber::new(-1, 0);
  b.iter(|| n.abs())
}

#[bench]
fn bench_abs_005(b: &mut Bencher) {
  let n = FeelNumber::new(123456, 6);
  b.iter(|| n.abs())
}

#[bench]
fn bench_abs_006(b: &mut Bencher) {
  let n = FeelNumber::new(-123456, 6);
  b.iter(|| n.abs())
}
