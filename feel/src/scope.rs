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

//! Implementation of the `FEEL` scope.

use crate::context::FeelContext;
use crate::values::Value;
use crate::{Name, QualifiedName};
use dmntk_common::Jsonify;
use std::cell::RefCell;
use std::fmt;

/// This macro creates a default scope.
#[macro_export]
macro_rules! scope {
  () => {{
    FeelScope::default()
  }};
}

/// The `FEEL` scope.
pub struct FeelScope {
  /// The stack of contexts.
  stack: RefCell<Vec<FeelContext>>,
}

impl Default for FeelScope {
  /// Creates a default [FeelScope] containing single default [FeelContext].
  fn default() -> Self {
    Self {
      stack: RefCell::new(vec![FeelContext::default()]),
    }
  }
}

impl From<FeelContext> for FeelScope {
  /// Creates a [FeelScope] from [FeelContext].
  fn from(ctx: FeelContext) -> Self {
    Self { stack: RefCell::new(vec![ctx]) }
  }
}

impl fmt::Display for FeelScope {
  /// Converts [FeelScope] to text.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}]", self.stack.borrow_mut().iter().map(|ctx| ctx.to_string()).collect::<Vec<String>>().join(", "))
  }
}

impl Jsonify for FeelScope {
  /// Converts this [FeelScope] to JSON text.
  fn jsonify(&self) -> String {
    format!("[{}]", self.stack.borrow_mut().iter().map(|ctx| ctx.to_string()).collect::<Vec<String>>().join(", "))
  }
}

impl FeelScope {
  /// Temporary - remove
  pub fn contexts(&self) -> Vec<FeelContext> {
    self.stack.borrow().clone()
  }

  /// Creates a new and empty [FeelScope].
  pub fn new() -> Self {
    Self { stack: RefCell::new(vec![]) }
  }

  /// Pushes a context on the top of the scope stack.
  pub fn push(&self, ctx: FeelContext) {
    self.stack.borrow_mut().push(ctx)
  }

  /// Appends the content of another scope at the end of this scope.
  pub fn append(&self, other: FeelScope) {
    self.stack.borrow_mut().append(&mut other.stack.borrow_mut());
  }

  /// Takes and returns a context from the top of the stack.
  pub fn pop(&self) -> Option<FeelContext> {
    self.stack.borrow_mut().pop()
  }

  /// Peeks a context from the top of the stack.
  /// If the stack is empty, the default context is returned.
  pub fn peek(&self) -> Option<FeelContext> {
    self.stack.borrow().last().cloned()
  }

  /// Returns a value of an entry with specified name.
  /// Entries are searched from the last to the first context,
  /// (from top to bottom of the stack).
  pub fn get_value(&self, name: &Name) -> Option<Value> {
    for context in self.stack.borrow().iter().rev() {
      if let Some(value) = context.get_entry(name) {
        return Some(value.clone());
      }
    }
    None
  }

  /// Searches for a value under so called `qualified` name build from
  /// multiple names passed as an argument.
  pub fn search(&self, names: &[Name]) -> Option<Value> {
    for context in self.stack.borrow().iter().rev() {
      if let Some(value) = context.search_deep(names) {
        return Some(value.clone());
      }
    }
    None
  }

  /// Searches for a value of an entry pointed by specified qualified name.
  pub fn search_entry(&self, qname: &QualifiedName) -> Option<Value> {
    for context in self.stack.borrow().iter().rev() {
      if let Some(value) = context.search_entry(qname) {
        return Some(value.clone());
      }
    }
    None
  }

  /// Sets a specified value for entry name in [FeelContext] placed on the top of the scope stack.
  pub fn set_value(&self, name: &Name, value: Value) {
    if let Some(context) = self.stack.borrow_mut().last_mut() {
      context.set_entry(name, value);
    }
  }

  /// Sets a null value for entry name in [FeelContext] placed on the top of the scope stack.
  pub fn set_name(&self, name: Name) {
    if let Some(context) = self.stack.borrow_mut().last_mut() {
      context.set_null(name);
    }
  }
}
