use super::*;

from_examples!(DMN_2_0112);

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{Age: 19}"#);
  let invocable_name = "Approval";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"["Best", "Standard"]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{Age: 13}"#);
  let invocable_name = "Approval";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"["Standard"]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{Age: 10}"#);
  let invocable_name = "Approval";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"["Standard"]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}
