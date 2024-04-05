use super::*;

from_examples!(DMN_3_0010);

#[test]
fn _0001() {
  let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "literalSimpleList", &ctx, r#"["a", "b", "c"]"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "literalNestedList", &ctx, r#"[["w", "x"], ["y"], ["z"]]"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "concatenate1", &ctx, r#"["a", "b", "c", "a", "b", "c"]"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "concatenate2", &ctx, r#"["a", "b", "c", "w", "x", "y", "z"]"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "concatenate3", &ctx, r#"["a", "b", "c", "w", "x", "y", "z"]"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "concatenate4", &ctx, r#"[["a", "b", "c"], ["w", "x"], ["y"], ["z"]]"#);
}
