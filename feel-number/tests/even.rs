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

mod common;

use dmntk_feel_number::FeelNumber;

#[test]
fn test_even_001() {
  assert!(num!(-4).even());
}

#[test]
fn test_even_002() {
  assert!(!num!(-3).even());
}

#[test]
fn test_even_003() {
  assert!(num!(-2).even());
}

#[test]
fn test_even_004() {
  assert!(!num!(-1).even());
}

#[test]
fn test_even_005() {
  assert!(num!(-0).even());
}

#[test]
fn test_even_006() {
  assert!(num!(0).even());
}

#[test]
fn test_even_007() {
  assert!(!num!(1).even());
}

#[test]
fn test_even_008() {
  assert!(num!(2).even());
}

#[test]
fn test_even_009() {
  assert!(!num!(3).even());
}

#[test]
fn test_even_010() {
  assert!(num!(4).even());
}

#[test]
fn test_even_011() {
  assert!(num!(0.0).even());
}

#[test]
fn test_even_012() {
  assert!(num!(2.000).even());
}

#[test]
fn test_even_013() {
  assert!(num!(-2.000).even());
}

#[test]
fn test_even_014() {
  assert!(!FeelNumber::new(41, 1).even());
}

#[test]
fn test_even_015() {
  assert!(!FeelNumber::new(41, 78).even());
}
