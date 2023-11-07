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

from_examples!(DMN_3_0011);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(
        r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#,
    );
    let invocable_name = "literalNestedList";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        invocable_name,
        &ctx,
        r#"[["a", "b"], ["b", "c"]]"#,
    );
    b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
    let ctx = context(
        r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#,
    );
    let invocable_name = "remove1";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        invocable_name,
        &ctx,
        r#"["a", "c"]"#,
    );
    b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
    let ctx = context(
        r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#,
    );
    let invocable_name = "insert3";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        invocable_name,
        &ctx,
        r#"[["o"], ["a", "b", "c"], ["p", "q"]]"#,
    );
    b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0004(b: &mut Bencher) {
    let ctx = context(
        r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#,
    );
    let invocable_name = "insert2";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        invocable_name,
        &ctx,
        r#"[["a", "b"], ["a", "b", "c"], ["b", "c"]]"#,
    );
    b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0005(b: &mut Bencher) {
    let ctx = context(
        r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#,
    );
    let invocable_name = "remove2";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        invocable_name,
        &ctx,
        r#"[["a", "b"]]"#,
    );
    b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0006(b: &mut Bencher) {
    let ctx = context(
        r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#,
    );
    let invocable_name = "insert1";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        invocable_name,
        &ctx,
        r#"["a", "x", "b", "c"]"#,
    );
    b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}
