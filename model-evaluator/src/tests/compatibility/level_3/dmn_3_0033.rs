use super::*;

from_examples!(DMN_3_0033);

#[test]
fn _0001() {
  let ctx = context(r#"{heights: [10, 20, 30]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "increase1", &ctx, r#"[11, 21, 31]"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{heights: [10, 20, 30], widths: [2, 3]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "areas", &ctx, r#"[20, 30, 40, 60, 60, 90]"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{factors: [2, 3, 5, 7, 11], value: 35}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "check factors", &ctx, r#"[false, false, true, true, false]"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{value: 10}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "multiples", &ctx, r#"[20, 30, 40, 50]"#);
}
