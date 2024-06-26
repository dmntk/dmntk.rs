use dmntk_feel::{scope, FeelScope};
use dmntk_feel_parser::AstNode;

#[test]
fn _0001() {
  let scope = scope!();
  let expected = AstNode::List(vec![
    AstNode::Numeric("1".to_string(), "".to_string()),
    AstNode::Numeric("2".to_string(), "".to_string()),
    AstNode::Numeric("3".to_string(), "".to_string()),
  ]);
  let actual = dmntk_feel_parser::parse_boxed_expression(&scope, "[1,2,3]", false).unwrap();
  assert_eq!(expected, actual);
}
