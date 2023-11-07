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

//! `FEEL` name implementation.

use dmntk_common::Jsonify;
use std::fmt;

/// `FEEL` name.
#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Clone)]
pub struct Name(String);

impl From<Vec<String>> for Name {
    /// Converts a vector of strings into [Name].
    fn from(value: Vec<String>) -> Self {
        Self::new(
            &value
                .iter()
                .map(|string| string.as_str())
                .collect::<Vec<&str>>(),
        )
    }
}

impl From<Vec<&str>> for Name {
    /// Converts a vector of string references into [Name].
    fn from(value: Vec<&str>) -> Self {
        Self::new(&value)
    }
}

impl From<String> for Name {
    /// Converts a [String] into [Name].
    fn from(value: String) -> Self {
        Self(value.trim().to_string())
    }
}

impl From<&str> for Name {
    /// Converts a reference to [str] into [Name].
    fn from(value: &str) -> Self {
        Self(value.trim().to_string())
    }
}

impl From<Name> for String {
    /// Converts [Name] to its [String] representation.
    fn from(value: Name) -> Self {
        value.0
    }
}

impl From<&Name> for String {
    /// Converts a reference to [Name] to its [String] representation.
    fn from(value: &Name) -> Self {
        value.0.clone()
    }
}

impl fmt::Display for Name {
    /// Implements [Display](fmt::Display) trait for [Name].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Jsonify for Name {
    /// Converts [Name] to its `JSON` representation.
    fn jsonify(&self) -> String {
        self.0.clone()
    }
}

impl Name {
    /// Creates a [Name] from string parts.
    pub fn new(parts: &[&str]) -> Self {
        let mut result = String::with_capacity(80);
        let mut current;
        let mut prev = false;
        for (index, part) in parts.iter().map(|s| s.trim()).enumerate() {
            current = matches!(part, "." | "/" | "-" | "'" | "+" | "*");
            if index > 0 && !prev && !current && !part.is_empty() {
                result.push(' ');
            }
            result.push_str(part);
            prev = current;
        }
        Self(result)
    }
}
