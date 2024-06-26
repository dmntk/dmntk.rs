use super::*;

from_examples!(DMN_2_0117);

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{Age: 19,RiskCategory: "Low",isAffordable: true}"#);
  let invocable_name = "Approval";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"{Rate: "Best", Status: "Approved"}"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{Age: 17,RiskCategory: "High",isAffordable: true}"#);
  let invocable_name = "Approval";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"{Rate: "Standard", Status: "Declined"}"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{Age: 19,RiskCategory: "Medium",isAffordable: true}"#);
  let invocable_name = "Approval";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"{Rate: "Standard", Status: "Approved"}"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}
