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

use super::*;

from_examples!(DMN_3_0035);

#[test]
fn _0001() {
    let ctx = context(r#"{B Value: 83,G Value: 65,R Value: 0}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "Profile of Color",
        &ctx,
        r##"{CMYK notation: {C: 100, K: 67, M: 22, Y: 0}, Hex notation: "#004153", RGB notation: {B: 83, G: 65, R: 0}}"##,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{B Value: 0,G Value: 0,R Value: 0}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "Profile of Color",
        &ctx,
        r##"{CMYK notation: {C: 0, K: 100, M: 0, Y: 0}, Hex notation: "#000000", RGB notation: {B: 0, G: 0, R: 0}}"##,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{B Value: 0,G Value: 0,R Value: 204}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        "Profile of Color",
        &ctx,
        r##"{CMYK notation: {C: 0, K: 20, M: 100, Y: 100}, Hex notation: "#CC0000", RGB notation: {B: 0, G: 0, R: 204}}"##,
    );
}
