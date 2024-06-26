use super::*;

from_examples!(DMN_3_0008);

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "listGen1", &ctx, r#"["a", "b", "c"]"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{a: "a", b: "b", c: "c"}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "listGen2", &ctx, r#"["a", "b", "c"]"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{b: "b", c: "c"}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "listGen3", &ctx, r#"["a", "b", "c"]"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{c: "c"}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "listGen4", &ctx, r#"["a", "b", "c"]"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{a: "a", b: "b", c: "c"}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "listGen5", &ctx, r#"["a", "b", "c"]"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "listGen6", &ctx, r#"["w", "x", "y", "z"]"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{wx: ["w", "x"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "listGen7", &ctx, r#"["w", "x", "y", "z"]"#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{a: "a", b: "b"}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "listGen8", &ctx, r#"["a", "b", "w", "x", "y", "z"]"#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{a: "a", b: "b", wx: ["w", "x"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "listGen9", &ctx, r#"["a", "b", "w", "x", "y", "z"]"#);
}

#[test]
fn _0010() {
  let ctx = context(r#"{c: "c", wx: ["w", "x"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "listGen10", &ctx, r#"["a", "b", "c", "w", "x", "y", "z"]"#);
}
