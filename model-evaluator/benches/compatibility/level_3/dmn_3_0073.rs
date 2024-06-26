use super::*;

from_examples!(DMN_3_0073);

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision_001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision_002";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision_003";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}
