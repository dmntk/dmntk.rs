use super::*;
use dmntk_feel::{scope, FeelType};

#[test]
fn _0001() {
  let node = AstNode::FunctionDefinition(
    Box::new(AstNode::FeelType(FeelType::Boolean)),
    Box::new(AstNode::FunctionBody(Box::new(AstNode::Numeric("1".into(), "0".into())), false)),
  );
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"null(expected formal parameters, actual value type is boolean)"#, result.ok().unwrap().to_string());
}
