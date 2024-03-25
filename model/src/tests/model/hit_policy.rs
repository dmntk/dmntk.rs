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

//! Hit policy tests.

use crate::model::{BuiltinAggregator, HitPolicy};
use dmntk_common::Result;

#[test]
fn test_display() {
  assert_eq!("U", format!("{}", HitPolicy::Unique));
  assert_eq!("A", format!("{}", HitPolicy::Any));
  assert_eq!("P", format!("{}", HitPolicy::Priority));
  assert_eq!("F", format!("{}", HitPolicy::First));
  assert_eq!("C", format!("{}", HitPolicy::Collect(BuiltinAggregator::List)));
  assert_eq!("C#", format!("{}", HitPolicy::Collect(BuiltinAggregator::Count)));
  assert_eq!("C+", format!("{}", HitPolicy::Collect(BuiltinAggregator::Sum)));
  assert_eq!("C<", format!("{}", HitPolicy::Collect(BuiltinAggregator::Min)));
  assert_eq!("C>", format!("{}", HitPolicy::Collect(BuiltinAggregator::Max)));
  assert_eq!("O", format!("{}", HitPolicy::OutputOrder));
  assert_eq!("R", format!("{}", HitPolicy::RuleOrder));
}

#[test]
fn test_debug() {
  assert_eq!("Unique", format!("{:?}", HitPolicy::Unique));
  assert_eq!("Any", format!("{:?}", HitPolicy::Any));
  assert_eq!("Priority", format!("{:?}", HitPolicy::Priority));
  assert_eq!("First", format!("{:?}", HitPolicy::First));
  assert_eq!("Collect(List)", format!("{:?}", HitPolicy::Collect(BuiltinAggregator::List)));
  assert_eq!("Collect(Count)", format!("{:?}", HitPolicy::Collect(BuiltinAggregator::Count)));
  assert_eq!("Collect(Sum)", format!("{:?}", HitPolicy::Collect(BuiltinAggregator::Sum)));
  assert_eq!("Collect(Min)", format!("{:?}", HitPolicy::Collect(BuiltinAggregator::Min)));
  assert_eq!("Collect(Max)", format!("{:?}", HitPolicy::Collect(BuiltinAggregator::Max)));
  assert_eq!("OutputOrder", format!("{:?}", HitPolicy::OutputOrder));
  assert_eq!("RuleOrder", format!("{:?}", HitPolicy::RuleOrder));
}

#[test]
fn test_equality() {
  assert!((HitPolicy::Unique == HitPolicy::Unique));
  assert!((HitPolicy::Any == HitPolicy::Any));
  assert!((HitPolicy::Priority == HitPolicy::Priority));
  assert!((HitPolicy::First == HitPolicy::First));
  assert!((HitPolicy::Collect(BuiltinAggregator::List) == HitPolicy::Collect(BuiltinAggregator::List)));
  assert!((HitPolicy::Collect(BuiltinAggregator::Count) == HitPolicy::Collect(BuiltinAggregator::Count)));
  assert!((HitPolicy::Collect(BuiltinAggregator::Sum) == HitPolicy::Collect(BuiltinAggregator::Sum)));
  assert!((HitPolicy::Collect(BuiltinAggregator::Min) == HitPolicy::Collect(BuiltinAggregator::Min)));
  assert!((HitPolicy::Collect(BuiltinAggregator::Max) == HitPolicy::Collect(BuiltinAggregator::Max)));
  assert!((HitPolicy::OutputOrder == HitPolicy::OutputOrder));
  assert!((HitPolicy::RuleOrder == HitPolicy::RuleOrder));
  assert!((HitPolicy::Unique != HitPolicy::First));
  assert!((HitPolicy::Any != HitPolicy::OutputOrder));
  assert!((HitPolicy::Priority != HitPolicy::Collect(BuiltinAggregator::Count)));
}

#[test]
fn test_try_from() {
  assert!(HitPolicy::Unique == "U".try_into().unwrap());
  assert!(HitPolicy::Any == "A".try_into().unwrap());
  assert!(HitPolicy::Priority == "P".try_into().unwrap());
  assert!(HitPolicy::First == "F".try_into().unwrap());
  assert!(HitPolicy::Collect(BuiltinAggregator::List) == "C".try_into().unwrap());
  assert!(HitPolicy::Collect(BuiltinAggregator::Count) == "C#".try_into().unwrap());
  assert!(HitPolicy::Collect(BuiltinAggregator::Sum) == "C+".try_into().unwrap());
  assert!(HitPolicy::Collect(BuiltinAggregator::Min) == "C<".try_into().unwrap());
  assert!(HitPolicy::Collect(BuiltinAggregator::Max) == "C>".try_into().unwrap());
  assert!(HitPolicy::OutputOrder == "O".try_into().unwrap());
  assert!(HitPolicy::RuleOrder == "R".try_into().unwrap());
  let hit_policy: Result<HitPolicy> = "K".try_into();
  assert!(hit_policy.is_err());
}
