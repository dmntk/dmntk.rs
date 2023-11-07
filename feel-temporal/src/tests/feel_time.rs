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

//! Test cases for `FEEL` time type.

use crate::{FeelTime, FeelZone};

#[test]
fn _0001() {
    let time_a = FeelTime::offset_opt(10, 11, 12, 0, 7).unwrap();
    let time_b =
        FeelTime::zone_opt(10, 11, 12, 0, FeelZone::Zone("Europe/Sofa".to_string())).unwrap();
    assert!(!(time_a == time_b));
}

#[test]
fn _0002() {
    let time_a = FeelTime::offset_opt(10, 11, 12, 0, 7).unwrap();
    let time_b =
        FeelTime::zone_opt(10, 11, 12, 0, FeelZone::Zone("Europe/Sofa".to_string())).unwrap();
    assert!((time_a - time_b).is_none());
}

#[test]
fn _0003() {
    assert!(FeelTime::local_opt(26, 11, 12, 0).is_none());
    assert!(FeelTime::local_opt(24, 1, 0, 0).is_none());
}

#[test]
fn _0004() {
    assert!(FeelTime::offset_opt(26, 11, 12, 0, 0).is_none());
    assert!(FeelTime::offset_opt(24, 1, 0, 0, 0).is_none());
}

#[test]
fn _0005() {
    assert!(FeelTime::zone_opt(26, 11, 12, 0, FeelZone::Utc).is_none());
    assert!(FeelTime::zone_opt(24, 1, 0, 0, FeelZone::Local).is_none());
}
