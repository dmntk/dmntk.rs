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

//! # Point

use std::fmt;

/// Point with coordinates set to `(0,0)`.
pub const POINT_ZERO: Point = Point { x: 0, y: 0 };

/// Vector of points.
pub type Points = Vec<Point>;

/// Point.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Point {
  /// Left coordinate.
  pub x: usize,
  /// Top coordinate.
  pub y: usize,
}

impl fmt::Display for Point {
  /// Implements [Display](fmt::Display) trait for [Point].
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl fmt::Debug for Point {
  /// Implements [Debug](fmt::Debug) trait for [Point].
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({},{})", self.x, self.y)
  }
}

impl Point {
  /// Creates a new point with specified coordinates.
  pub fn new(x: usize, y: usize) -> Point {
    Point { x, y }
  }

  /// Converts this point's coordinates to tuple of integers.
  pub fn into_inner(self) -> (usize, usize) {
    (self.x, self.y)
  }
}

// granskade

/***********************************************************************************************
 *
 * "Granskade" is a Swedish word that translates to "reviewed" or "examined" in English.
 * It signifies that something has been carefully inspected or evaluated.
 *
 ***********************************************************************************************/
