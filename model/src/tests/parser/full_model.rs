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

use crate::model::DmnElement;
use crate::parse;
use dmntk_examples::DMN_FULL;

#[test]
fn _0001() {
  let definitions = parse(DMN_FULL).unwrap();
  assert_eq!("_id_definitions", definitions.id());
  //------------------------------------------------------------------------------------------------
  // ITEM DEFINITIONS
  //------------------------------------------------------------------------------------------------
  assert_eq!(0, definitions.item_definitions().len());
  //------------------------------------------------------------------------------------------------
  // EXTENSION ELEMENTS IN DEFINITIONS
  //------------------------------------------------------------------------------------------------
  assert_eq!(0, definitions.extension_elements().len());
  //------------------------------------------------------------------------------------------------
  // DRG ELEMENTS
  //------------------------------------------------------------------------------------------------
  assert_eq!(1, definitions.drg_elements().len());
  assert_eq!(1, definitions.decisions().len());
  assert_eq!(0, definitions.input_data().len());
  //------------------------------------------------------------------------------------------------
  // DMNDI
  //------------------------------------------------------------------------------------------------
  assert!(definitions.dmndi().is_none());
}

/// Covers all cloning and debugging functions of `Definitions`.
#[test]
#[allow(clippy::redundant_clone)]
fn _0002() {
  let definitions = parse(DMN_FULL).unwrap();
  let cloned_definitions = definitions.clone();
  assert_eq!("_id_definitions", cloned_definitions.id());
  let expected = format!("{definitions:?}");
  assert_eq!(expected, format!("{cloned_definitions:?}"));
}
