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

//! # URI reference
//!
//! This [HRef] struct utilizes an **href** attribute whose value must be a valid URI reference
//! [RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986), where the path components
//! may be absolute or relative, the reference has no query component,
//! and the fragment consists of the value of the **id** of the referenced DMN element.

use self::errors::*;
use crate::DmntkError;
use uriparse::{RelativeReference, URIReference, URI};

/// URI reference used for utilizing `href` attribute.
#[derive(Debug, Clone)]
pub struct HRef {
  /// Namespace built from URI's path components.
  namespace: Option<String>,
  /// DMN element's identifier built from URI's fragment.
  id: String,
}

impl HRef {
  /// Returns the optional namespace.
  pub fn namespace(&self) -> Option<&String> {
    self.namespace.as_ref()
  }

  /// Returns the identifier.
  pub fn id(&self) -> &str {
    &self.id
  }
}

impl TryFrom<&str> for HRef {
  type Error = DmntkError;
  /// Converts [HRef] from string.
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if let Ok(reference) = RelativeReference::try_from(value) {
      if reference.has_query() {
        return Err(err_invalid_reference(value));
      }
      let id = reference.fragment().ok_or_else(|| err_invalid_reference_no_fragment(value))?.to_string();
      let path = reference.path().to_string().trim().trim_end_matches('/').to_string();
      let namespace = if path.is_empty() { None } else { Some(path) };
      return Ok(Self { namespace, id });
    }
    if let Ok(uri_reference) = URIReference::try_from(value) {
      if let Ok(uri) = URI::try_from(uri_reference) {
        if uri.has_query() {
          return Err(err_invalid_reference(value));
        }
        let id = uri.fragment().ok_or_else(|| err_invalid_reference_no_fragment(value))?.to_string();
        let path = uri.into_base_uri().to_string().trim().trim_end_matches('/').to_string();
        let namespace = if path.is_empty() { None } else { Some(path) };
        return Ok(Self { namespace, id });
      }
    }
    Err(err_invalid_reference(value))
  }
}

mod errors {
  use crate::{DmntkError, ToErrorMessage};

  /// Errors reported by [HRef](crate::href::HRef).
  #[derive(ToErrorMessage)]
  struct HRefError(String);

  /// Creates an error indicating an invalid reference.
  pub fn err_invalid_reference(s: &str) -> DmntkError {
    HRefError(format!("invalid reference: '{s}'")).into()
  }

  /// Creates an error indicating the missing fragment.
  pub fn err_invalid_reference_no_fragment(s: &str) -> DmntkError {
    HRefError(format!("no fragment in reference: '{s}'")).into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_relative_references() {
    assert_eq!(r#"Err(DmntkError("<HRefError> no fragment in reference: ''"))"#, format!("{:?}", HRef::try_from("")));
    assert_eq!(
      r#"Err(DmntkError("<HRefError> no fragment in reference: 'documents'"))"#,
      format!("{:?}", HRef::try_from("documents"))
    );
    assert_eq!(
      r#"Ok(HRef { namespace: Some("documents"), id: "_b51ac78b-fd76-42fc-a12d-aad7150c9278" })"#,
      format!("{:?}", HRef::try_from("documents#_b51ac78b-fd76-42fc-a12d-aad7150c9278"))
    );
    assert_eq!(
      r#"Ok(HRef { namespace: None, id: "_b51ac78b-fd76-42fc-a12d-aad7150c9278" })"#,
      format!("{:?}", HRef::try_from("#_b51ac78b-fd76-42fc-a12d-aad7150c9278"))
    );
    assert_eq!(
      r#"Err(DmntkError("<HRefError> invalid reference: 'documents?name=Introduction#_b51ac78b-fd76-42fc-a12d-aad7150c9278'"))"#,
      format!("{:?}", HRef::try_from("documents?name=Introduction#_b51ac78b-fd76-42fc-a12d-aad7150c9278"))
    );
  }

  #[test]
  fn test_absolute_references() {
    assert_eq!(r#"Err(DmntkError("<HRefError> no fragment in reference: ''"))"#, format!("{:?}", HRef::try_from("")));
    assert_eq!(
      r#"Err(DmntkError("<HRefError> invalid reference: '                                               '"))"#,
      format!("{:?}", HRef::try_from("                                               "))
    );
    assert_eq!(
      r#"Ok(HRef { namespace: Some("https://dmntk.io/documents"), id: "_b51ac78b-fd76-42fc-a12d-aad7150c9278" })"#,
      format!("{:?}", HRef::try_from("https://dmntk.io/documents#_b51ac78b-fd76-42fc-a12d-aad7150c9278"))
    );
    assert_eq!(
      r#"Err(DmntkError("<HRefError> no fragment in reference: 'https://dmntk.io/documents'"))"#,
      format!("{:?}", HRef::try_from("https://dmntk.io/documents"))
    );
    assert_eq!(
      r#"Err(DmntkError("<HRefError> invalid reference: 'https::\\/dmntk.io/documents#id'"))"#,
      format!("{:?}", HRef::try_from("https::\\/dmntk.io/documents#id"))
    );
  }
}
