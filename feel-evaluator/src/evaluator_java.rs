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

//! # Evaluator for Java external functions

use dmntk_feel::dto::ValueDto;
use dmntk_feel::value_null;
use dmntk_feel::values::Value;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static CLIENT: Lazy<reqwest::blocking::Client> = Lazy::new(reqwest::blocking::Client::new);

const JAVA_RPC_SERVER_URL: &str = "http://127.0.0.1:22023/api/rest/v1/rpc/evaluate";

#[derive(Serialize)]
struct RequestDto {
  /// Name of the Java class, where called method is defined.
  #[serde(rename = "className")]
  class_name: String,
  /// Name of the static method to be called.
  #[serde(rename = "methodName")]
  method_name: String,
  /// List of parameter types of the called method.
  #[serde(rename = "parameterTypes")]
  parameter_types: Vec<String>,
  /// Arguments to be passed to called method.
  #[serde(rename = "arguments")]
  argument_values: Vec<ValueDto>,
}

#[derive(Deserialize)]
struct ResponseDto {
  /// Response payload when calling external function succeeds.
  #[serde(rename = "data")]
  data: Option<ValueDto>,
  /// Error message on failure.
  #[serde(rename = "error")]
  error: Option<String>,
}

/// Evaluates external Java function.
pub fn evaluate_external_java_function(class_name: &str, method_signature: &str, arguments: &[Value]) -> Value {
  let mut parts = method_signature.trim().split('(');
  let Some(method_name) = parts.next() else {
    return value_null!("no method name in method signature");
  };
  let Some(parameter_type_names) = parts.next() else {
    return value_null!("no parameter types in method signature");
  };
  let parameter_types: Vec<String> = parameter_type_names
    .trim()
    .trim_end_matches(')')
    .split(',')
    .filter_map(|s| if s.trim().is_empty() { None } else { Some(s.trim().to_string()) })
    .collect();
  if parameter_types.len() != arguments.len() {
    return value_null!(
      "the number of parameter types ({}) differs from the number of arguments ({})",
      parameter_types.len(),
      arguments.len()
    );
  }
  let mut argument_values = vec![];
  for argument in arguments {
    match ValueDto::try_from(argument) {
      Ok(value_dto) => argument_values.push(value_dto),
      Err(reason) => return value_null!("{}", reason.to_string()),
    };
  }
  let request_dto = RequestDto {
    class_name: class_name.to_string(),
    method_name: method_name.to_string(),
    parameter_types,
    argument_values,
  };
  match CLIENT.post(JAVA_RPC_SERVER_URL).json(&request_dto).send() {
    Ok(response) => match response.json::<ResponseDto>() {
      Ok(response_dto) => {
        if let Some(reason) = response_dto.error {
          value_null!("{}", reason)
        } else if let Some(value_dto) = response_dto.data {
          match Value::try_from(&value_dto) {
            Ok(value) => value,
            Err(reason) => value_null!("{}", reason),
          }
        } else {
          value_null!("no data in response")
        }
      }
      Err(reason) => value_null!("{}", reason),
    },
    Err(reason) => value_null!("{}", reason),
  }
}
