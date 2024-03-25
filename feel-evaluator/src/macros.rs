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

/// Builds `null` value with a message informing about invalid argument type.
macro_rules! invalid_argument_type {
  ($function:literal, $expected:expr, $actual:expr) => {{
    use dmntk_feel::value_null;
    use std::file;
    use std::path::Path;
    value_null!(
      Path::new(file!()).file_stem().unwrap().to_string_lossy(),
      $function,
      "{}",
      format!("invalid argument type, expected {}, actual type is {}", $expected, $actual)
    )
  }};
}

pub(crate) use invalid_argument_type;

/// Builds `null` value with a message informing about invalid parameter count.
macro_rules! invalid_number_of_parameters {
  ($expected:expr, $actual:expr) => {{
    use dmntk_feel::value_null;
    value_null!("expected {} parameters, actual number of parameters is {}", $expected, $actual)
  }};
}

pub(crate) use invalid_number_of_parameters;
