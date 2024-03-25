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

use super::super::point::*;

#[test]
fn test_point_zero() {
  let p = POINT_ZERO;
  assert_eq!(p.x, 0);
  assert_eq!(p.y, 0);
}

#[test]
fn test_point_new() {
  let p = Point::new(1, 2);
  assert_eq!(p.x, 1);
  assert_eq!(p.y, 2);
  p.assert_receiver_is_total_eq();
}

#[test]
fn test_point_display() {
  assert_eq!("(10,20)", format!("{}", Point::new(10, 20)));
  assert_eq!("(0,0)", format!("{}", Point::new(0, 0)));
}

#[test]
fn test_point_debug() {
  assert_eq!("(10,20)", format!("{:?}", Point::new(10, 20)));
  assert_eq!("(0,0)", format!("{:?}", Point::new(0, 0)));
}

#[test]
fn test_point_compare() {
  let p1 = Point::new(1, 2);
  let p2 = Point::new(1, 2);
  let p3 = Point::new(2, 1);
  assert!((p1 == p2));
  assert!((p1 != p3));
}

#[test]
fn test_point_into_inner() {
  assert_eq!((21, 22), Point::new(21, 22).into_inner());
}

#[test]
#[allow(clippy::clone_on_copy)]
fn test_point_clone() {
  let p1 = Point::new(1, 2);
  let p2 = p1.clone();
  assert!((p1 == p2));
}
