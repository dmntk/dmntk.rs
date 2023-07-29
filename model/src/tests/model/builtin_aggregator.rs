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

//! BuiltinAggregator tests.

use crate::model::BuiltinAggregator;

#[test]
fn test_display() {
  assert_eq!("C", format!("{}", BuiltinAggregator::List));
  assert_eq!("C#", format!("{}", BuiltinAggregator::Count));
  assert_eq!("C+", format!("{}", BuiltinAggregator::Sum));
  assert_eq!("C<", format!("{}", BuiltinAggregator::Min));
  assert_eq!("C>", format!("{}", BuiltinAggregator::Max));
}

#[test]
fn test_debug() {
  assert_eq!("List", format!("{:?}", BuiltinAggregator::List));
  assert_eq!("Count", format!("{:?}", BuiltinAggregator::Count));
  assert_eq!("Sum", format!("{:?}", BuiltinAggregator::Sum));
  assert_eq!("Min", format!("{:?}", BuiltinAggregator::Min));
  assert_eq!("Max", format!("{:?}", BuiltinAggregator::Max));
}

#[test]
fn test_equality() {
  assert!((BuiltinAggregator::List == BuiltinAggregator::List));
  assert!((BuiltinAggregator::Count == BuiltinAggregator::Count));
  assert!((BuiltinAggregator::Sum == BuiltinAggregator::Sum));
  assert!((BuiltinAggregator::Min == BuiltinAggregator::Min));
  assert!((BuiltinAggregator::Max == BuiltinAggregator::Max));
  assert!((BuiltinAggregator::Max != BuiltinAggregator::Min));
  assert!((BuiltinAggregator::Sum != BuiltinAggregator::Count));
  assert!((BuiltinAggregator::List != BuiltinAggregator::Sum));
}
