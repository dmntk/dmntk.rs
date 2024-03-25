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

mod ascii_model;
mod compatibility;
mod decision_tables;

use std::fs;
use std::fs::File;
use std::io::Write;

/// Name of the target directory.
const TARGET_DIR: &str = "../target/gendoc";

/// Utility function for generating HTML file for decision table defined as text.
fn gen_html_from_model(model: &str, output_file_name: &str) {
  let definitions = dmntk_model::parse(model).expect("parsing model failed");
  let html = crate::dmn_model_to_html(&definitions);
  assert_eq!("<!DOCTYPE html>", &html[0..15]);
  fs::create_dir_all(TARGET_DIR).expect("creating target directories failed");
  let mut file = File::create(format!("{TARGET_DIR}/{output_file_name}.html")).expect("creating output file failed");
  file.write_all(html.as_bytes()).expect("saving output file failed");
}

/// Utility macro for generating HTML file for DMN™ model.
macro_rules! export_model {
  ($t:tt) => {{
    gen_html_from_model(dmntk_examples::$t, stringify!($t));
  }};
}

use export_model;
