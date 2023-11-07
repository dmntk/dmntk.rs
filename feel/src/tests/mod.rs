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

mod bif;
mod context;
mod dto;
mod function;
mod names;
mod qualified_names;
mod scope;
mod types;
mod values;

use crate::{FeelType, Name};
use once_cell::sync::Lazy;

const T_ANY: &FeelType = &FeelType::Any;
const T_BOOLEAN: &FeelType = &FeelType::Boolean;
const T_NUMBER: &FeelType = &FeelType::Number;
const T_DATE: &FeelType = &FeelType::Date;
const T_DATE_TIME: &FeelType = &FeelType::DateTime;
const T_DAYS_AND_TIME_DURATION: &FeelType = &FeelType::DaysAndTimeDuration;
const T_NULL: &FeelType = &FeelType::Null;
const T_STRING: &FeelType = &FeelType::String;
const T_TIME: &FeelType = &FeelType::Time;
const T_YEARS_AND_MONTHS_DURATION: &FeelType = &FeelType::YearsAndMonthsDuration;

static NAME_A: Lazy<Name> = Lazy::new(|| Name::from("a"));
static NAME_B: Lazy<Name> = Lazy::new(|| Name::from("b"));
static NAME_C: Lazy<Name> = Lazy::new(|| Name::from("c"));
static NAME_D: Lazy<Name> = Lazy::new(|| Name::from("d"));
static T_LIST_A: Lazy<FeelType> = Lazy::new(|| FeelType::list(T_NUMBER));
static T_LIST_B: Lazy<FeelType> = Lazy::new(|| FeelType::list(T_BOOLEAN));
static T_LIST_C: Lazy<FeelType> = Lazy::new(|| FeelType::list(T_STRING));
static T_LIST_D: Lazy<FeelType> = Lazy::new(|| FeelType::list(T_ANY));
static T_CONTEXT_A: Lazy<FeelType> = Lazy::new(|| FeelType::context(&[(&NAME_A, T_NUMBER)]));
static T_CONTEXT_B: Lazy<FeelType> = Lazy::new(|| FeelType::context(&[(&NAME_B, T_BOOLEAN)]));
static T_CONTEXT_C: Lazy<FeelType> = Lazy::new(|| FeelType::context(&[(&NAME_A, T_STRING)]));
static T_CONTEXT_A_B: Lazy<FeelType> = Lazy::new(|| FeelType::context(&[(&NAME_A, T_NUMBER), (&NAME_B, T_BOOLEAN)]));
static T_CONTEXT_A_B_C: Lazy<FeelType> = Lazy::new(|| FeelType::context(&[(&NAME_A, T_NUMBER), (&NAME_B, T_BOOLEAN), (&NAME_C, T_STRING)]));
static T_FUNCTION_A: Lazy<FeelType> = Lazy::new(|| FeelType::function(&[T_NUMBER.clone(), T_NUMBER.clone()], T_NUMBER));
static T_FUNCTION_B: Lazy<FeelType> = Lazy::new(|| FeelType::function(&[T_NUMBER.clone(), T_NUMBER.clone()], T_BOOLEAN));
static T_FUNCTION_C: Lazy<FeelType> = Lazy::new(|| FeelType::function(&[T_NUMBER.clone()], T_STRING));
static T_FUNCTION_D: Lazy<FeelType> = Lazy::new(|| FeelType::function(&[], T_ANY));
static T_FUNCTION_E: Lazy<FeelType> = Lazy::new(|| FeelType::function(&[], T_STRING));
static T_FUNCTION_F: Lazy<FeelType> = Lazy::new(|| FeelType::function(&[T_ANY.clone()], T_STRING));
static T_FUNCTION_G: Lazy<FeelType> = Lazy::new(|| FeelType::function(&[T_STRING.clone()], T_STRING));
static T_RANGE_A: Lazy<FeelType> = Lazy::new(|| FeelType::range(T_NUMBER));
static T_RANGE_B: Lazy<FeelType> = Lazy::new(|| FeelType::range(T_DATE));
