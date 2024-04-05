use super::*;

#[test]
fn _0001() {
  let scope = &te_scope("{}");
  te_bool(false, scope, r#"some n in [1,2,3] satisfies n > 1.5"#, true);
}

#[test]
fn _0002() {
  let scope = &te_scope("{}");
  te_bool(false, scope, r#"some n in [1,2,3] satisfies n > 10.5"#, false);
}

#[test]
fn _0003() {
  let scope = &te_scope("{}");
  te_bool(false, scope, r#"some n in 1 satisfies n > 10.5"#, false);
}

#[test]
fn _0004() {
  let scope = &te_scope("{}");
  te_bool(false, scope, r#"some n in 12.58 satisfies n > 10.5"#, true);
}

#[test]
fn _0005() {
  let scope = FeelScope::default();
  let node = AstNode::Some(Box::new(AstNode::Name("n".into())), Box::new(AstNode::Name("n".into())));
  assert_eq!(
    r#"<FeelEvaluatorError> expected AST node AstNode::QuantifiedContexts, actual AST node is Name(Name("n"))"#,
    crate::evaluate(&scope, &node).err().unwrap().to_string()
  );
}

#[test]
fn _0006() {
  let scope = FeelScope::default();
  let node = AstNode::Some(Box::new(AstNode::QuantifiedContexts(vec![])), Box::new(AstNode::Name("n".into())));
  assert_eq!(
    r#"<FeelEvaluatorError> expected AST node AstNode::Satisfies, actual AST node is Name(Name("n"))"#,
    crate::evaluate(&scope, &node).err().unwrap().to_string()
  );
}
