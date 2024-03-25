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

use crate::FeelDateTime;
use chrono::{DateTime, FixedOffset};
use dmntk_common::Result;

#[test]
fn _0001() {
  let date_time: FeelDateTime = "2023-01-10T24:00:00".try_into().unwrap();
  assert_eq!("2023-01-11T00:00:00", date_time.to_string());
}

#[test]
fn _0002() {
  let date_time: Result<FeelDateTime> = "999999999-01-10T24:00:00".try_into();
  assert_eq!(
    "<TemporalError> invalid date and time literal '999999999-01-10T24:00:00'",
    date_time.err().unwrap().to_string()
  );
}

#[test]
fn _0003() {
  let date_time: Result<FeelDateTime> = "2023-02-09T24:01:00".try_into();
  assert_eq!("<TemporalError> invalid date and time literal '2023-02-09T24:01:00'", date_time.err().unwrap().to_string());
}

#[test]
fn _0004() {
  let date_time: Result<FeelDateTime> = "2023-02-09T24:01:00@Europe/Sofa".try_into();
  assert_eq!(
    "<TemporalError> invalid date and time literal '2023-02-09T24:01:00@Europe/Sofa'",
    date_time.err().unwrap().to_string()
  );
}

#[test]
fn _0005() {
  let date_time_1: FeelDateTime = "2023-01-10T24:00:00".try_into().unwrap();
  let date_time_2: FeelDateTime = "2023-01-10T24:00:00Z".try_into().unwrap();
  assert!(!(date_time_1 == date_time_2));
}

#[test]
fn _0006() {
  let feel_date_time: FeelDateTime = "99999999-01-01T00:00:00".try_into().unwrap();
  let date_time: Result<DateTime<FixedOffset>> = feel_date_time.try_into();
  assert_eq!(
    "<TemporalError> conversion from FEEL date '99999999-01-01T00:00:00' to DateTime<FixedOffset> failed, see issue #? for details",
    date_time.err().unwrap().to_string()
  );
}
