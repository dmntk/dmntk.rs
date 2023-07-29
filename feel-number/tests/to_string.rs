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
fn test_to_string_001() {
  eqs!("49", num!(49));
}

#[test]
fn test_to_string_002() {
  eqs!("49.0", FeelNumber::new(490, 1).to_string());
}

#[test]
fn test_to_string_003() {
  eqs!("4900", FeelNumber::new(4900, 0).to_string());
}

#[test]
fn test_to_string_004() {
  eqs!("50", FeelNumber::new(50, 0).to_string());
}

#[test]
fn test_to_string_005() {
  eqs!("50", num!(50));
}

#[test]
fn test_to_string_006() {
  eqs!("50.5", FeelNumber::new(505, 1).to_string());
}

#[test]
fn test_to_string_007() {
  eqs!("50.50", FeelNumber::new(5050, 2).to_string());
}

#[test]
fn test_to_string_008() {
  assert_eq!("50.10", format!("{:.2}", num!(50.1)));
}

#[test]
fn test_to_string_009() {
  assert_eq!("50.123456", format!("{:.6}", num!(50.123456789)));
}

#[test]
fn test_to_string_010() {
  assert_eq!("  50.1", format!("{:>6}", num!(50.1)));
}
