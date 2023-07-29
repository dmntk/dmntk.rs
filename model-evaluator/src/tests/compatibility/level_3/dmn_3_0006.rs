/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2015-2023 Dariusz Depta, Engos Software
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
 * Copyright (c) 2015-2023 Dariusz Depta, Engos Software
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

use super::*;

from_examples!(DMN_3_0006);

const INPUT_DATA: &str = r#"
  {
    DeptTable:  [
      {
        manager: "Smith",
        name: "Sales",
        number: 10
      },
      {
        manager: "Jones",
        name: "Finance",
        number: 20
      },
      {
        manager: "King",
        name: "Engineering",
        number: 30
      }
    ],
    EmployeeTable:  [
      {
        deptNum: 10,
        id: "7792",
        name: "Clark"
      },
      {
        deptNum: 10,
        id: "7934",
        name: "Miller"
      },
      {
        deptNum: 20,
        id: "7976",
        name: "Adams"
      },
      {
        deptNum: 20,
        id: "7902",
        name: "Ford"
      },
      {
        deptNum: 30,
        id: "7900",
        name: "James"
      }
    ],
    LastName: "Clark"
  }
"#;

#[test]
fn _0001() {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Join", &ctx, r#""Smith""#);
}
