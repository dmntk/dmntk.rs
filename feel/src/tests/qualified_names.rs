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

use crate::{Name, QualifiedName};

#[test]
fn test_qualified_name() {
  let name_a = Name::new(&["a", "+", "b"]);
  let name_b = Name::new(&["b", "-", "c"]);
  let name_c = Name::new(&["c", "/", "d"]);
  let name_d = Name::new(&["d", "*", "e"]);
  let name_e = Name::new(&["e", ".", "f"]);
  let name_f = Name::new(&["f", "'", "g"]);
  let qname = QualifiedName::new(&[]);
  assert_eq!("", qname.to_string().as_str());
  let qname = QualifiedName::new(&[&name_a]);
  assert_eq!("a+b", qname.to_string().as_str());
  let qname = QualifiedName::new(&[&name_a, &name_b]);
  assert_eq!("a+b.b-c", qname.to_string().as_str());
  let qname = QualifiedName::new(&[&name_a, &name_b, &name_c]);
  assert_eq!("a+b.b-c.c/d", qname.to_string().as_str());
  let qname = QualifiedName::new(&[&name_a, &name_b, &name_c, &name_d]);
  assert_eq!("a+b.b-c.c/d.d*e", qname.to_string().as_str());
  let qname = QualifiedName::new(&[&name_a, &name_b, &name_c, &name_d, &name_e]);
  assert_eq!("a+b.b-c.c/d.d*e.e.f", qname.to_string().as_str());
  let qname = QualifiedName::new(&[&name_a, &name_b, &name_c, &name_d, &name_e, &name_f]);
  assert_eq!("a+b.b-c.c/d.d*e.e.f.f'g", qname.to_string().as_str());
}

#[test]
fn test_push() {
  let name_a = Name::new(&["a"]);
  let name_b = Name::new(&["b"]);
  let name_c = Name::new(&["c"]);
  let mut qname = QualifiedName::new(&[]);
  assert_eq!("", qname.to_string().as_str());
  qname.push(name_a);
  assert_eq!("a", qname.to_string().as_str());
  qname.push(name_b);
  assert_eq!("a.b", qname.to_string().as_str());
  qname.push(name_c);
  assert_eq!("a.b.c", qname.to_string().as_str());
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_clone() {
  let name_a = Name::new(&["a"]);
  let name_b = Name::new(&["b"]);
  let name_c = Name::new(&["c"]);
  let qname = QualifiedName::new(&[&name_a, &name_b, &name_c]);
  assert_eq!("a.b.c", qname.to_string().as_str());
  let qname_clone = qname.clone();
  assert_eq!("a.b.c", qname_clone.to_string().as_str());
}

#[test]
fn test_debug() {
  let name_a = Name::new(&["a"]);
  let name_b = Name::new(&["b"]);
  let name_c = Name::new(&["c"]);
  let qname = QualifiedName::new(&[&name_a, &name_b, &name_c]);
  assert_eq!(r#"QualifiedName([Name("a"), Name("b"), Name("c")])"#, format!("{qname:?}"));
  qname.assert_receiver_is_total_eq();
}

#[test]
fn test_compare() {
  let name_a = Name::new(&["a"]);
  let name_b = Name::new(&["b"]);
  let name_c = Name::new(&["c"]);
  let qname_a = QualifiedName::new(&[&name_a, &name_b]);
  let qname_b = QualifiedName::new(&[&name_a, &name_b]);
  let qname_c = QualifiedName::new(&[&name_a, &name_b, &name_c]);
  assert!((qname_a == qname_b));
  assert!((qname_a != qname_c));
}
