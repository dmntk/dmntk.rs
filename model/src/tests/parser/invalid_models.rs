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

use crate::parse;
use crate::tests::parser::input_files::*;

#[test]
fn _0001() {
    let definitions = parse(T_DMN_0001);
    assert!(definitions.is_err());
    assert_eq!(
        r#"<ModelParserError> 'Python' is not a valid function kind, accepted values are: 'FEEL', 'Java', 'PMML'"#,
        format!("{}", definitions.err().unwrap())
    )
}

#[test]
fn _0002() {
    let definitions = parse(T_DMN_0002);
    assert!(definitions.is_err());
    assert_eq!(
        r#"<ModelParserError> 'LAST' is not a valid hit policy, allowed values are: 'UNIQUE', 'FIRST', 'PRIORITY', 'ANY', 'COLLECT', 'RULE ORDER', 'OUTPUT ORDER'"#,
        format!("{}", definitions.err().unwrap())
    )
}

#[test]
fn _0003() {
    let definitions = parse(T_DMN_0003);
    assert!(definitions.is_err());
    assert_eq!(
        r#"<ModelParserError> 'AVG' is not a valid aggregation, allowed values are: 'COUNT', 'SUM', 'MIN', 'MAX'"#,
        format!("{}", definitions.err().unwrap())
    )
}

#[test]
fn _0004() {
    let definitions = parse(T_DMN_0004);
    assert!(definitions.is_err());
    assert_eq!(
        r#"<ModelParserError> required input expression in decision table's input clause is missing"#,
        format!("{}", definitions.err().unwrap())
    )
}

#[test]
fn _0005() {
    let definitions = parse(T_DMN_0005);
    assert!(definitions.is_err());
    assert_eq!(
        r#"<ModelParserError> required expression instance is missing"#,
        format!("{}", definitions.err().unwrap())
    )
}

#[test]
fn _0006() {
    let definitions = parse(T_DMN_0006);
    assert!(definitions.is_err());
    assert_eq!(
        r#"<ModelParserError> number of elements in a row differs from the number of columns defined in a relation"#,
        format!("{}", definitions.err().unwrap())
    )
}

#[test]
fn _0007() {
    let definitions = parse(T_DMN_0007);
    assert!(definitions.is_err());
    assert_eq!(
        r#"<ModelParserError> parsing model from XML failed with reason: the root node was opened but never closed"#,
        format!("{}", definitions.err().unwrap())
    )
}

#[test]
fn _0008() {
    let definitions = parse(T_DMN_0008);
    assert!(definitions.is_err());
    assert_eq!(
        r#"<ModelParserError> unexpected XML node, expected: definitions, actual: definition"#,
        format!("{}", definitions.err().unwrap())
    )
}

#[test]
fn _0009() {
    let definitions = parse(T_DMN_0009);
    assert!(definitions.is_err());
    assert_eq!(
        r#"<ModelParserError> expected value for mandatory attribute 'namespace' in node 'definitions' at [2:1]"#,
        format!("{}", definitions.err().unwrap())
    )
}

#[test]
fn _0010() {
    let definitions = parse(T_DMN_0010);
    assert!(definitions.is_err());
    assert_eq!(
        r#"<ModelParserError> expected mandatory child node 'variable' in parent node 'decision' at [11:5]"#,
        format!("{}", definitions.err().unwrap())
    )
}

#[test]
fn _0011() {
    let definitions = parse(T_DMN_0011);
    assert!(definitions.is_err());
    assert_eq!(
        r#"<ModelParserError> expected mandatory text content in node 'text'"#,
        format!("{}", definitions.err().unwrap())
    )
}

#[test]
fn _0012() {
    let definitions = parse(T_DMN_0012);
    assert!(definitions.is_err());
    assert_eq!(
        r#"<ModelParserError> conversion to valid color value failed with reason: number too large to fit in target type"#,
        format!("{}", definitions.err().unwrap())
    )
}

#[test]
fn _0013() {
    let definitions = parse(T_DMN_0013);
    assert!(definitions.is_err());
    assert_eq!(
        r#"<ModelParserError> conversion to valid double value failed with reason: invalid float literal"#,
        format!("{}", definitions.err().unwrap())
    )
}

#[test]
fn _0014() {
    let definitions = parse(T_DMN_0014);
    assert!(definitions.is_err());
    assert_eq!(
        r#"<ModelParserError> expected mandatory child node 'text' in parent node 'outputEntry' at [31:17]"#,
        format!("{}", definitions.err().unwrap())
    )
}

#[test]
fn _0015() {
    let definitions = parse(T_DMN_0015);
    assert!(definitions.is_ok());
}

#[test]
fn _0016() {
    let definitions = parse(T_DMN_0016);
    assert!(definitions.is_err());
    assert_eq!(
        r#"<ModelParserError> required child node 'Bounds' in parent node 'DMNShape' is missing"#,
        format!("{}", definitions.err().unwrap())
    )
}
