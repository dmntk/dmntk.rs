use super::*;

from_examples!(DMN_3_0030);

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r##"{ stringInputA:  "feel", stringInputB:  "#" }"##);
  let invocable_name = "simple function invocation";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r##""feel#feel#""##);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r##"{ stringInputA:  "feel", stringInputB:  "#" }"##);
  let invocable_name = "named function invocation";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r##""#feel#feel""##);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}
