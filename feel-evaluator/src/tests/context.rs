use super::*;
use dmntk_feel::scope;

#[test]
fn _0001() {
  let input = r#"
  {
    Full Name: "John Doe",
    Full Name: "John Travis"
  }
  "#;
  let node = dmntk_feel_parser::parse_context(&scope!(), input, false).unwrap();
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"null(duplicated context entry key: Full Name)"#, result.ok().unwrap().to_string());
}

#[test]
fn _0002() {
  let input = r#"
  {
    "Full Name": "John Doe",
    "Address": "Atlanta"
  }
  "#;
  let node = dmntk_feel_parser::parse_context(&scope!(), input, false).unwrap();
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"{Address: "Atlanta", Full Name: "John Doe"}"#, result.ok().unwrap().to_string());
}

#[test]
fn _0003() {
  let node = AstNode::Context(vec![AstNode::String("key".to_string())]);
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"null(expected context entry, actual value type is string)"#, result.ok().unwrap().to_string());
}

#[test]
fn _0004() {
  let node = AstNode::Context(vec![AstNode::ContextEntry(
    Box::new(AstNode::String("key".to_string())),
    Box::new(AstNode::String("value".to_string())),
  )]);
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"null(expected context entry key, actual value type is string)"#, result.ok().unwrap().to_string());
}

#[test]
fn _0005() {
  let input = r#" { convert: function(x: context <age: number, name: stringa>) x } "#;
  let node = dmntk_feel_parser::parse_context(&scope!(), input, false).unwrap();
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"{convert: null(expected type of the formal parameter)}"#, result.ok().unwrap().to_string());
}

#[test]
fn _0006() {
  let node = AstNode::ContextType(vec![AstNode::ContextTypeEntry(
    Box::new(AstNode::String("key".to_string())),
    Box::new(AstNode::String("value".to_string())),
  )]);
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"null(expected a name in context type entry)"#, result.ok().unwrap().to_string());
}

#[test]
fn _0007() {
  let node = AstNode::ContextType(vec![AstNode::String("key".to_string())]);
  let result = crate::evaluate(&scope!(), &node);
  assert_eq!(r#"null(expected context type entry, actual value type is string)"#, result.ok().unwrap().to_string());
}
