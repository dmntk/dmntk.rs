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

mod common;

use dmntk_feel_number::FeelNumber;

#[test]
fn test_rem_001() {
  eqs!("2", FeelNumber::new(12, 0) % FeelNumber::new(5, 0));
}

#[test]
fn test_rem_002() {
  eqs!("3", FeelNumber::new(-12, 0) % FeelNumber::new(5, 0));
}

#[test]
fn test_rem_003() {
  eqs!("-3", FeelNumber::new(12, 0) % FeelNumber::new(-5, 0));
}

#[test]
fn test_rem_004() {
  eqs!("-2", FeelNumber::new(-12, 0) % FeelNumber::new(-5, 0));
}

#[test]
fn test_rem_005() {
  eqs!("1.1", FeelNumber::new(101, 1) % FeelNumber::new(45, 1));
}

#[test]
fn test_rem_006() {
  eqs!("3.4", FeelNumber::new(-101, 1) % FeelNumber::new(45, 1));
}

#[test]
fn test_rem_007() {
  eqs!("-3.4", FeelNumber::new(101, 1) % FeelNumber::new(-45, 1));
}

#[test]
fn test_rem_008() {
  eqs!("-1.1", FeelNumber::new(-101, 1) % FeelNumber::new(-45, 1));
}

#[test]
fn test_rem_009() {
  let mut x = FeelNumber::new(101, 1);
  x %= FeelNumber::new(-45, 1);
  eqs!("-3.4", x);
}
