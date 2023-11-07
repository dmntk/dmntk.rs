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

//! # Closure

use crate::{Name, QualifiedName};
use std::collections::btree_set::Iter;
use std::collections::BTreeSet;
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Closure(BTreeSet<QualifiedName>);

impl fmt::Display for Closure {
    /// Converts a closure to string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl From<Vec<QualifiedName>> for Closure {
    /// Creates a [Closure] from a vector of [QualifiedNames](QualifiedName).
    fn from(value: Vec<QualifiedName>) -> Self {
        Self(value.iter().cloned().collect())
    }
}

impl Closure {
    /// Returns an iterator over closure items.
    pub fn iter(&self) -> Iter<QualifiedName> {
        self.0.iter()
    }

    /// Removes a closure item with specified name.
    pub fn remove(&mut self, name: Name) {
        let qname: QualifiedName = name.into();
        self.0.remove(&qname);
    }
}
