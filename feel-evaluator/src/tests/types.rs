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

use dmntk_feel::{scope, FeelScope, FeelType};
use dmntk_feel_parser::AstNode;

#[test]
fn _0001() {
    let node = AstNode::FunctionType(
        Box::new(AstNode::ParameterTypes(vec![
            AstNode::FeelType(FeelType::Number),
            AstNode::FeelType(FeelType::String),
        ])),
        Box::new(AstNode::FeelType(FeelType::Boolean)),
    );
    let result = crate::evaluate(&scope!(), &node);
    assert_eq!(
        r#"type(function<number, string>->boolean)"#,
        result.ok().unwrap().to_string()
    );
}

#[test]
fn _0002() {
    let node = AstNode::FunctionType(
        Box::new(AstNode::ParameterTypes(vec![
            AstNode::FeelType(FeelType::Number),
            AstNode::FeelType(FeelType::String),
        ])),
        Box::new(AstNode::Numeric("0".into(), "0".into())),
    );
    let result = crate::evaluate(&scope!(), &node);
    assert_eq!(
        r#"null(expected function's result type)"#,
        result.ok().unwrap().to_string()
    );
}

#[test]
fn _0003() {
    let node = AstNode::FunctionType(
        Box::new(AstNode::Numeric("0".into(), "0".into())),
        Box::new(AstNode::FeelType(FeelType::Boolean)),
    );
    let result = crate::evaluate(&scope!(), &node);
    assert_eq!(
        r#"null(expected function's parameter types)"#,
        result.ok().unwrap().to_string()
    );
}

#[test]
fn _0004() {
    let node = AstNode::ListType(Box::new(AstNode::FeelType(FeelType::Boolean)));
    let result = crate::evaluate(&scope!(), &node);
    assert_eq!(r#"type(list<boolean>)"#, result.ok().unwrap().to_string());
}

#[test]
fn _0005() {
    let node = AstNode::ListType(Box::new(AstNode::Numeric("0".into(), "0".into())));
    let result = crate::evaluate(&scope!(), &node);
    assert_eq!(
        r#"null(expected a feel type)"#,
        result.ok().unwrap().to_string()
    );
}

#[test]
fn _0006() {
    let node = AstNode::RangeType(Box::new(AstNode::FeelType(FeelType::Number)));
    let result = crate::evaluate(&scope!(), &node);
    assert_eq!(r#"type(range<number>)"#, result.ok().unwrap().to_string());
}

#[test]
fn _0007() {
    let node = AstNode::RangeType(Box::new(AstNode::Numeric("0".into(), "0".into())));
    let result = crate::evaluate(&scope!(), &node);
    assert_eq!(
        r#"null(expected a feel type)"#,
        result.ok().unwrap().to_string()
    );
}
