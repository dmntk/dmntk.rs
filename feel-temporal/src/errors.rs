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

//! Implementation of FEEL temporal errors.

use crate::FeelDate;
use dmntk_common_1::{DmntkError, ToErrorMessage};
use dmntk_feel_number::FeelNumber;

/// FEEL temporal error.
#[derive(ToErrorMessage)]
struct TemporalError(String);

pub fn err_invalid_date(y: FeelNumber, m: FeelNumber, d: FeelNumber) -> DmntkError {
    TemporalError(format!("invalid date {y}-{m}-{d}")).into()
}

pub fn err_invalid_feel_date(date: FeelDate) -> DmntkError {
    TemporalError(format!(
        "invalid date {}-{}-{}",
        date.year(),
        date.month(),
        date.day()
    ))
    .into()
}

pub fn err_invalid_date_literal(s: &str) -> DmntkError {
    TemporalError(format!("invalid date literal '{s}'")).into()
}

pub fn err_invalid_time_literal(s: &str) -> DmntkError {
    TemporalError(format!("invalid time literal '{s}'")).into()
}

pub fn err_invalid_date_time_literal(s: &str) -> DmntkError {
    TemporalError(format!("invalid date and time literal '{s}'")).into()
}

pub fn err_date_time_conversion_failed(s: &str) -> DmntkError {
    TemporalError(format!(
        "conversion from FEEL date '{s}' to DateTime<FixedOffset> failed, see issue #? for details"
    ))
    .into()
}

pub fn err_invalid_years_and_months_duration_literal(s: &str) -> DmntkError {
    TemporalError(format!("invalid years and months literal '{s}'")).into()
}

pub fn err_invalid_time_zone_offset(offset: i32) -> DmntkError {
    TemporalError(format!("invalid time-zone offset '{offset}'")).into()
}

pub fn err_invalid_date_and_time_duration_literal(literal: String) -> DmntkError {
    TemporalError(format!("invalid date and time duration literal: {literal}")).into()
}
