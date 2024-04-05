use super::*;

#[test]
fn _0001() {
  let node = AstNode::Path(Box::new(AstNode::Name("a".into())), Box::new(AstNode::Name("b".into())));
  let scope = te_scope(r#"{ a: { b: 10 } }"#);
  let result = crate::evaluate(&scope, &node);
  assert_eq!(r#"10"#, result.ok().unwrap().to_string());
}

#[test]
fn _0002() {
  let node = AstNode::Path(Box::new(AstNode::Name("a".into())), Box::new(AstNode::Name("b".into())));
  let scope = te_scope(r#"{ a: { c: 10 } }"#);
  let result = crate::evaluate(&scope, &node);
  assert_eq!(r#"null(build_path: no entry b in context: {c: 10})"#, result.ok().unwrap().to_string());
}

#[test]
fn _0003() {
  let node = AstNode::Path(Box::new(AstNode::Name("a".into())), Box::new(AstNode::Name("b".into())));
  let scope = te_scope(r#"{ a: [{ b: 10 }, { b: 11 }, { b: 12 }] }"#);
  let result = crate::evaluate(&scope, &node);
  assert_eq!(r#"[10, 11, 12]"#, result.ok().unwrap().to_string());
}

#[test]
fn _0004() {
  let node = AstNode::Path(Box::new(AstNode::Name("a".into())), Box::new(AstNode::Name("b".into())));
  let scope = te_scope(r#"{ a: [{ b: 10 }, { b: 11 }, 12 ] }"#);
  let result = crate::evaluate(&scope, &node);
  assert_eq!(r#"null(build_path: no context in list)"#, result.ok().unwrap().to_string());
}

#[test]
fn _0005() {
  let node = AstNode::Path(
    Box::new(AstNode::Name("a".into())),
    Box::new(AstNode::Path(Box::new(AstNode::Name("b".into())), Box::new(AstNode::Name("days".into())))),
  );
  let scope = te_scope(r#"{ a: [{ b: @"P1DT5H" }, { b: @"P2DT6H" }, { b: @"P3DT7H" }] }"#);
  let result = crate::evaluate(&scope, &node);
  assert_eq!(r#"[1, 2, 3]"#, result.ok().unwrap().to_string());
}

#[test]
fn _0006() {
  let node = AstNode::Path(
    Box::new(AstNode::Name("a".into())),
    Box::new(AstNode::Path(Box::new(AstNode::Boolean(false)), Box::new(AstNode::Name("days".into())))),
  );
  let scope = te_scope(r#"{ a: [{ b: @"P1DT5H" }, { b: @"P2DT6H" }, { b: @"P3DT7H" }] }"#);
  let result = crate::evaluate(&scope, &node);
  assert_eq!(
    r#"<FeelEvaluatorError> unexpected AST node in evaluator builder expected Name, found Boolean(false)"#,
    result.err().unwrap().to_string()
  );
}

#[test]
fn _0007() {
  let node = AstNode::Path(Box::new(AstNode::Name("a".into())), Box::new(AstNode::Boolean(false)));
  let scope = te_scope(r#"{ a: [{ b: @"P1DT5H" }, { b: @"P2DT6H" }, { b: @"P3DT7H" }] }"#);
  let result = crate::evaluate(&scope, &node);
  assert_eq!(
    r#"<FeelEvaluatorError> unexpected AST node in evaluator builder expected Path or Name, found: Boolean(false)"#,
    result.err().unwrap().to_string()
  );
}
