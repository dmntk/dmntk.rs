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

//! # Examples of decision tables defined as Unicode text
//!
//! File naming convention for horizontal decision tables (rules as rows):
//!
//! ```text
//! ┌────────────── table orientation: h - horizontal (rules as rows)
//! │ ┌──────────── information item name: 0 - absent, 1 - present
//! │ │┌─────────── output label: 0 - absent, 1 - present
//! │ ││┌────────── allowed values: 0 - absent, 1 - present
//! │ │││┌───────── number of input clauses: 0, 1, 2...
//! │ ││││┌──────── number of output clauses: 1, 2, 3...
//! │ │││││┌─────── number of annotation clauses: 0, 1, 2...  
//! H_000010
//! ```

mod crosstab;
mod horizontal;
mod vertical;

pub use crosstab::*;
pub use horizontal::*;
pub use vertical::*;
