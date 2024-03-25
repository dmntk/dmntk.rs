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

//! # Conversion utilities for namespaces

use url::Url;

/// Converts an URI into its RDNN-like equivalent.
///
/// Unless the input namespace URI is invalid or does not contain a domain name,
/// this function returns its RDNN-like equivalent. Domain name segments are reversed,
/// path segments order is preserved, all segments are joined with
/// the forward slash `/` character.
///
/// Returns `None` for namespace URIs that can not be converted to its RDNN-like equivalent.
///
/// # Examples
///
/// ```
/// use dmntk_common::to_rdnn;
///
/// let rdnn = to_rdnn("https://dmntk.io/system-1/component-1");
/// assert_eq!(Some("io/dmntk/system-1/component-1".to_string()), rdnn);
///
/// let rdnn = to_rdnn("https://dmntk.io");
/// assert_eq!(Some("io/dmntk".to_string()), rdnn);
///
/// let rdnn = to_rdnn("dmntk.io");
/// assert_eq!(None, rdnn);
///
/// ```
pub fn to_rdnn(input: &str) -> Option<String> {
  let Ok(url) = Url::parse(input) else {
    return None;
  };
  let segments = url.path_segments()?;
  let mut path_segments = segments.map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<&str>>();
  let domain = url.domain()?;
  let mut domain_segments = domain.split('.').collect::<Vec<&str>>();
  domain_segments.reverse();
  domain_segments.append(&mut path_segments);
  Some(domain_segments.join("/"))
}

#[cfg(test)]
mod tests {
  use super::*;

  fn assert_rdnn(expected: &str, actual: Option<String>) {
    let value = actual.unwrap();
    let url = format!("https://dmntk.io/{}", value);
    assert!(Url::parse(&url).is_ok());
    assert_eq!(expected, value);
  }

  #[test]
  fn test_to_rdnn() {
    assert_rdnn(
      "au/com/montera/www/spec/DMN/0099-arithmetic-negation",
      to_rdnn("https://www.montera.com.au/spec/DMN/0099-arithmetic-negation"),
    );
    assert_rdnn(
      "au/com/montera/www/spec/DMN/0099-arithmetic-negation",
      to_rdnn("https://www.montera.com.au/spec/DMN/0099-arithmetic-negation#_427d2e23-d096-47d5-b861-ebb7f37f461e"),
    );
    assert_rdnn("au/com/montera/www", to_rdnn("https://www.montera.com.au"));
    assert_rdnn("io/dmntk/model-2/decision-5", to_rdnn("https://dmntk.io/model-2/decision-5"));
    assert_eq!(None, to_rdnn("https://www.montera.com.au|error"));
    assert_eq!(None, to_rdnn("https://127.0.0.1"));
    assert_eq!(None, to_rdnn("montera.com"));
    assert_eq!(None, to_rdnn("data:text/plain,HelloWorld"));
  }
}

// granskade
