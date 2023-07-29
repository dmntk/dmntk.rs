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

use super::*;

from_examples!(DMN_N_0019);

static_context!(
  CTX,
  r#"{
    "Flight List": [
      { "Flight Number": "UA123",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T18:00:00", "Arrival": @"2017-01-01T19:00:00", "Capacity": 5, "Status": "cancelled" },
      { "Flight Number": "UA456",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T19:00:00", "Arrival": @"2017-01-01T20:00:00", "Capacity": 2, "Status": "scheduled" },
      { "Flight Number": "UA789",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T21:00:00", "Arrival": @"2017-01-01T23:00:00", "Capacity": 2, "Status": "scheduled" },
      { "Flight Number": "UA1001", "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T23:00:00", "Arrival": @"2017-01-02T05:00:00", "Capacity": 0, "Status": "scheduled" },
      { "Flight Number": "UA1111", "From": "SFO", "To": "LAX", "Departure": @"2017-01-01T23:00:00", "Arrival": @"2017-01-02T05:00:00", "Capacity": 2, "Status": "scheduled" }
    ],
    "Passenger List": [
      { "Name": "Tom",   "Status": "bronze", "Miles": 10,     "Flight Number": "UA123" },
      { "Name": "Igor",  "Status": "gold",   "Miles": 50000,  "Flight Number": "UA123" },
      { "Name": "Jenny", "Status": "gold",   "Miles": 500000, "Flight Number": "UA123" },
      { "Name": "Harry", "Status": "gold",   "Miles": 100000, "Flight Number": "UA123" },
      { "Name": "Dick",  "Status": "silver", "Miles": 100,    "Flight Number": "UA123" }
    ]
  }"#
);

#[test]
fn _0001() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "Rebooked Passengers",
    &CTX,
    r#"[{Flight Number: "UA456", Miles: 500000, Name: "Jenny", Status: "gold"}, {Flight Number: "UA456", Miles: 100000, Name: "Harry", Status: "gold"}, {Flight Number: "UA789", Miles: 50000, Name: "Igor", Status: "gold"}, {Flight Number: "UA789", Miles: 100, Name: "Dick", Status: "silver"}, {Flight Number: null, Miles: 10, Name: "Tom", Status: "bronze"}]"#,
  );
}

#[test]
fn _0002() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "Prioritized Waiting List",
    &CTX,
    r#"[{Flight Number: "UA123", Miles: 500000, Name: "Jenny", Status: "gold"}, {Flight Number: "UA123", Miles: 100000, Name: "Harry", Status: "gold"}, {Flight Number: "UA123", Miles: 50000, Name: "Igor", Status: "gold"}, {Flight Number: "UA123", Miles: 100, Name: "Dick", Status: "silver"}, {Flight Number: "UA123", Miles: 10, Name: "Tom", Status: "bronze"}]"#,
  );
}
