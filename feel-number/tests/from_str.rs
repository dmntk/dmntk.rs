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
fn test_from_str_001() {
  eqs!("12345.6789", "12345.6789".parse::<FeelNumber>().unwrap());
}

#[test]
fn test_from_str_002() {
  assert!("1234a5".parse::<FeelNumber>().is_err());
}

#[test]
fn test_from_str_003() {
  eqs!("<FeelNumberError> invalid number literal '1234a5'", "1234a5".parse::<FeelNumber>().unwrap_err().to_string());
}

#[test]
fn test_from_str_004() {
  eqs!("12300", "1.23E+4".parse::<FeelNumber>().unwrap());
}

#[test]
fn test_from_str_005() {
  eqs!("0.00000000000000000000001", "1E-23".parse::<FeelNumber>().unwrap());
}

#[test]
fn test_from_str_006() {
  eqs!("0.00000000000000001234567", "1.234567E-17".parse::<FeelNumber>().unwrap());
}
