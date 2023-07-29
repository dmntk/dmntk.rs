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

use super::super::*;
use dmntk_feel::scope;

#[test]
fn _0001() {
  te_string(false, &scope!(), r##"replace("abcd","(ab)|(a)","[1=$1][2=$2]")"##, r##"[1=ab][2=]cd"##);
}

#[test]
fn _0002() {
  te_string(false, &scope!(), r##"replace("a","[b-z]","#")"##, r##"a"##);
}

#[test]
fn _0003() {
  te_string(false, &scope!(), r##"replace("a","[a-z]","#")"##, r##"#"##);
}

#[test]
fn _0004() {
  te_string(false, &scope!(), r##"replace("abc","def","#")"##, r##"abc"##);
}

#[test]
fn _0005() {
  te_string(false, &scope!(), r##"replace("abc","e","#")"##, r##"abc"##);
}

#[test]
fn _0006() {
  te_string(false, &scope!(), r##"replace("foobar","^fo*b*","#")"##, r##"#ar"##);
}

#[test]
fn _0007() {
  te_string(false, &scope!(), r##"replace("abc",".^[d-z]","#")"##, r##"abc"##);
}

#[test]
fn _0008() {
  te_string(false, &scope!(), r##"replace("abracadabra","bra","*")"##, r##"a*cada*"##);
}

#[test]
fn _0009() {
  te_string(false, &scope!(), r##"replace("abracadabra","a.*a","*")"##, r##"*"##);
}

#[test]
fn _0010() {
  te_string(false, &scope!(), r##"replace("abracadabra","a.*?a","*")"##, r##"*c*bra"##);
}

#[test]
fn _0011() {
  te_string(false, &scope!(), r##"replace("abracadabra","a","")"##, r##"brcdbr"##);
}

#[test]
fn _0012() {
  te_string(false, &scope!(), r##"replace("AAAA","A+","b")"##, r##"b"##);
}

#[test]
fn _0013() {
  te_string(false, &scope!(), r##"replace("AAAA","A+?","b")"##, r##"bbbb"##);
}

#[test]
fn _0014() {
  te_string(false, &scope!(), r##"replace("darted","^(.*?)d(.*)$","$1$2")"##, r##"arted"##);
}

#[test]
fn _0015() {
  te_string(false, &scope!(), r##"replace("darted","^(.*?)d(.*)$","$1c$2")"##, r##"carted"##);
}

#[test]
fn _0016() {
  te_string(false, &scope!(), r##"replace("reluctant","r.*?t","X")"##, r##"Xant"##);
}

#[test]
fn _0017() {
  te_string(false, &scope!(), r##"replace("0123456789","(\d{3})(\d{3})(\d{4})","($1) $2-$3")"##, r##"(012) 345-6789"##);
}

#[test]
fn _0018() {
  te_string(false, &scope!(), r##"replace("abc","[a-z]","#","")"##, r##"###"##);
}

#[test]
fn _0019() {
  te_string(false, &scope!(), r##"replace("a.b.c.","[a-z]","#","s")"##, r##"#.#.#."##);
}

#[test]
fn _0020() {
  te_string(false, &scope!(), r##"replace("abc","[A-Z]","#","i")"##, r##"###"##);
}

#[test]
fn _0021() {
  te_string(false, &scope!(), r##"replace("abc","[a-z]","#","s")"##, r##"###"##);
}

#[test]
fn _0022() {
  te_string(false, &scope!(), r##"replace("a b c d ","[a-z]","#","x")"##, r##"# # # # "##);
}

#[test]
fn _0023() {
  te_string(false, &scope!(), r##"replace("a b c d ","[a-z]","#")"##, r##"# # # # "##);
}

#[test]
fn _0024() {
  te_string(false, &scope!(), r##"replace("abc",".^[d-z]*","smix")"##, r##"abc"##);
}

#[test]
fn _0025() {
  te_string(false, &scope!(), r##"replace(input:"abc",pattern:"[a-z]",replacement:"#")"##, r##"###"##);
}

#[test]
fn _0026() {
  te_string(false, &scope!(), r##"replace(input:"abc",pattern:"[A-Z]",replacement:"#",flags:"")"##, r##"abc"##);
}

#[test]
fn _0027() {
  te_string(false, &scope!(), r##"replace(input:"abc",pattern:"[A-Z]",replacement:"#",flags:"i")"##, r##"###"##);
}

#[test]
fn _0028() {
  te_string(false, &scope!(), r##"replace(input:"abc",pattern:".^[d-z]*",replacement:"#",flags:"smix")"##, r##"abc"##);
}

#[test]
fn _0029() {
  te_string(false, &scope!(), r##"replace("a\b\c","\\","\\\\","q")"##, r##"a\\b\\c"##);
}

#[test]
fn _0030() {
  te_string(false, &scope!(), r##"replace("a/b/c","/","$","q")"##, r##"a$b$c"##);
}

#[test]
fn _0031() {
  te_string(false, &scope!(), r##"replace("abc","[A-Z]","#","all unknown but i")"##, r##"###"##);
}

#[test]
fn _0032() {
  te_string(false, &scope!(), r##"replace(replace("anbncnz","n","\u000A"),"[A-C]\n","u","i")"##, r##"uuuz"##);
}

#[test]
fn _0033() {
  te_string(false, &scope!(), r##"replace("a\u000Ab\u000Ac\u000A","[A-Z]\n","u","i")"##, r##"uuu"##);
}

#[test]
fn _0034() {
  te_null(false, &scope!(), r#"replace()"#, r#"expected 3,4 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0035() {
  te_null(false, &scope!(), r#"replace("abc")"#, r#"expected 3,4 parameters, actual number of parameters is 1"#);
}

#[test]
fn _0036() {
  te_null(false, &scope!(), r#"replace("abc","b")"#, r#"expected 3,4 parameters, actual number of parameters is 2"#);
}

#[test]
fn _0037() {
  te_null(
    false,
    &scope!(),
    r#"replace("abc","b","c","d","e")"#,
    r#"expected 3,4 parameters, actual number of parameters is 5"#,
  );
}

#[test]
fn _0038() {
  te_null(
    false,
    &scope!(),
    r##"replace(i:"abc",pattern:".^[d-z]*",replacement:"#",flags:"smix")"##,
    r#"parameter 'input' not found"#,
  );
}

#[test]
fn _0039() {
  te_null(
    false,
    &scope!(),
    r##"replace(input:"abc",p:".^[d-z]*",replacement:"#",flags:"smix")"##,
    r#"parameter 'pattern' not found"#,
  );
}

#[test]
fn _0040() {
  te_null(
    false,
    &scope!(),
    r##"replace(input:"abc",pattern:".^[d-z]*",r:"#",flags:"smix")"##,
    r#"parameter 'replacement' not found"#,
  );
}

#[test]
fn _0041() {
  te_null(false, &scope!(), r#"replace(10,"[a-z]","A")"#, r#"replace: input must be a string"#);
}

#[test]
fn _0042() {
  te_null(false, &scope!(), r#"replace("abc1",10,"A")"#, r#"replace: pattern must be a string"#);
}

#[test]
fn _0043() {
  te_null(false, &scope!(), r#"replace("abc1","[a-z]",10)"#, r#"replace: replacement must be a string"#);
}

#[test]
fn _0044() {
  te_null(false, &scope!(), r#"replace("abc1","[a-z","A")"#, r#"replace: invalid pattern"#);
}
