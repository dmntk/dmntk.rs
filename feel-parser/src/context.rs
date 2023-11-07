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

//! Implementation of the parsing context.

use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::{FeelType, Name};
use std::borrow::Borrow;
use std::collections::{BTreeMap, HashSet};
use std::fmt;

/// Types of entries in parsing context.
pub enum ParsingEntry {
    /// Parsing entry representing a context.
    Context(ParsingContext),
    /// Parsing entry representing a variable.
    Variable,
}

impl fmt::Display for ParsingEntry {
    /// Converts parsing entry into text representation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParsingEntry::Context(ctx) => ctx.to_string(),
                ParsingEntry::Variable => "<v>".to_string(),
            }
        )
    }
}

impl From<FeelType> for ParsingEntry {
    /// Converts a reference to feel type into parsing entry.
    fn from(value: FeelType) -> Self {
        Self::from(&value)
    }
}

impl From<&FeelType> for ParsingEntry {
    /// Converts a feel type into parsing entry.
    fn from(value: &FeelType) -> Self {
        match value {
            FeelType::Context(feel_type_ctx) => {
                let mut entries = BTreeMap::new();
                for (name, feel_type) in feel_type_ctx {
                    entries.insert(name.to_owned(), feel_type.into());
                }
                ParsingEntry::Context(ParsingContext(entries))
            }
            FeelType::List(feel_type_items) => {
                let feel_type = feel_type_items.borrow();
                if let FeelType::Context(_) = feel_type {
                    Self::from(feel_type)
                } else {
                    ParsingEntry::Variable
                }
            }
            _ => ParsingEntry::Variable,
        }
    }
}

/// Parsing context.
#[derive(Default)]
pub struct ParsingContext(BTreeMap<Name, ParsingEntry>);

impl fmt::Display for ParsingContext {
    /// Converts parsing context into text representation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|(name, entry)| { format!(r#"{name}: {entry}"#) })
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl From<FeelContext> for ParsingContext {
    /// Converts feel context into parsing context.
    fn from(ctx: FeelContext) -> Self {
        Self::from(&ctx)
    }
}

impl From<&FeelContext> for ParsingContext {
    /// Converts a reference to feel context into parsing context.
    fn from(value: &FeelContext) -> Self {
        let mut entries = BTreeMap::new();
        for (name, value) in value.iter() {
            match value {
                Value::Context(ctx) => {
                    entries.insert(name.to_owned(), ParsingEntry::Context(ctx.into()));
                }
                list @ Value::List(_) => {
                    entries.insert(name.to_owned(), list.type_of().into());
                }
                Value::FeelType(feel_type) => {
                    entries.insert(name.to_owned(), feel_type.into());
                }
                _ => {
                    entries.insert(name.to_owned(), ParsingEntry::Variable);
                }
            }
        }
        Self(entries)
    }
}

impl ParsingContext {
    /// Places a specified name in this parsing context.
    pub fn set_name(&mut self, name: Name) {
        self.0.insert(name, ParsingEntry::Variable);
    }

    /// Places parsing context under specified name.
    pub fn set_context(&mut self, name: Name, ctx: ParsingContext) {
        self.0.insert(name, ParsingEntry::Context(ctx));
    }

    /// Returns a list of flattened keys for this parsing context.
    pub fn flattened_keys(&self) -> HashSet<String> {
        let mut keys: HashSet<String> = HashSet::new();
        for (key, value) in self.0.iter() {
            keys.insert(key.into());
            if let ParsingEntry::Context(sub_ctx) = value {
                let sub_keys = sub_ctx.flattened_keys();
                if !sub_keys.is_empty() {
                    for sub_key in sub_keys {
                        keys.insert(sub_key.clone());
                        keys.insert(format!("{key} . {sub_key}"));
                    }
                }
            }
        }
        keys
    }
}
