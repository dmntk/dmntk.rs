use super::*;

from_examples!(DMN_2_0114);

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{NumOfYears: 5}"#);
  let invocable_name = "CarInsurance";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"64.32"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{NumOfYears: 3}"#);
  let invocable_name = "CarInsurance";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"98.83"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{NumOfYears: 4}"#);
  let invocable_name = "CarInsurance";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"98.83"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}
