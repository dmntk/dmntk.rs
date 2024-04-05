use super::*;

from_examples!(DMN_3_0030);

#[test]
fn _0001() {
  let ctx = context(r##"{stringInputA: "feel", stringInputB: "#"}"##);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "simple function invocation", &ctx, r##""feel#feel#""##);
}

#[test]
fn _0002() {
  let ctx = context(r##"{stringInputA: "feel", stringInputB: "#"}"##);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "named function invocation", &ctx, r##""#feel#feel""##);
}
