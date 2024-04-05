use super::*;
use dmntk_feel::scope;

#[test]
fn _0001() {
  let node = AstNode::Numeric("2".into(), "56".into());
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"2.56"#, result.ok().unwrap().to_string());
}

#[test]
fn _0002() {
  let node = AstNode::Numeric("2".into(), "5a6".into());
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"null(failed to convert text '2.5a6' into number)"#, result.ok().unwrap().to_string());
}
