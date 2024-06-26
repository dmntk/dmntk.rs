//! Random UUID generator

use uuid::Uuid;

/// Returns a string representation of a random UUID v4.
///
/// # Example
///
/// The string should be 36 characters long.
///
/// ```
/// use dmntk_common::gen_id;
///
/// assert_eq!(36, gen_id().len());
/// ```
/// # References
///
/// * [Version 4 UUIDs in RFC4122](https://www.rfc-editor.org/rfc/rfc4122#section-4.4)
///
pub fn gen_id() -> String {
  Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_valid_references() {
    let id = gen_id();
    for (i, ch) in id.chars().enumerate() {
      if matches!(i, 8 | 13 | 18 | 23) {
        assert_eq!(ch, '-');
      } else {
        assert!(matches!(ch, 'a'..='f' | '0'..='9'))
      }
    }
  }
}

// granskade
