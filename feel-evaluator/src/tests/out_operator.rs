use super::*;
use dmntk_feel::scope;

#[test]
fn _0001() {
  let node = AstNode::Out(
    Box::new(AstNode::Numeric("21".to_string(), "".to_string())),
    Box::new(AstNode::Numeric("21".to_string(), "".to_string())),
  );
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"21"#, result.ok().unwrap().to_string());
}

#[test]
fn _0002() {
  let node = AstNode::Out(Box::new(AstNode::Numeric("21".to_string(), "".to_string())), Box::new(AstNode::FormalParameters(vec![])));
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"null"#, result.ok().unwrap().to_string());
}
