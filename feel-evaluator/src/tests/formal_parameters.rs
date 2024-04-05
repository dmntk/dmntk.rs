use super::*;
use dmntk_feel::{scope, FeelType};

#[test]
fn _0001() {
  let node = AstNode::FormalParameter(Box::new(AstNode::ParameterName("n".into())), Box::new(AstNode::FeelType(FeelType::Boolean)));
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!("FormalParameter", result.ok().unwrap().to_string());
}

#[test]
fn _0002() {
  let node = AstNode::FormalParameter(Box::new(AstNode::Name("n".into())), Box::new(AstNode::FeelType(FeelType::Boolean)));
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!("null(expected name of the formal parameter)", result.ok().unwrap().to_string());
}

#[test]
fn _0003() {
  let parameter_x = AstNode::FormalParameter(Box::new(AstNode::ParameterName("x".into())), Box::new(AstNode::FeelType(FeelType::Number)));
  let parameter_y = AstNode::FormalParameter(Box::new(AstNode::ParameterName("y".into())), Box::new(AstNode::FeelType(FeelType::Number)));
  let node = AstNode::FormalParameters(vec![parameter_x, parameter_y]);
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!("FormalParameters", result.ok().unwrap().to_string());
}

#[test]
fn _0004() {
  let parameter_x = AstNode::FormalParameter(Box::new(AstNode::ParameterName("x".into())), Box::new(AstNode::FeelType(FeelType::Number)));
  let parameter_y = AstNode::FormalParameter(Box::new(AstNode::ParameterName("y".into())), Box::new(AstNode::FeelType(FeelType::Number)));
  let parameter_invalid = AstNode::Numeric("1".into(), "0".into());
  let node = AstNode::FormalParameters(vec![parameter_x, parameter_y, parameter_invalid]);
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!("null(expected formal parameter, actual value type is: number)", result.ok().unwrap().to_string());
}
