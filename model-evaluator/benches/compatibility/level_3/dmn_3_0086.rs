use super::*;

static MODEL_EVALUATOR: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0086_IMPORT, DMN_3_0086]));

const NAMESPACE: &str = "https://dmntk.io/3_0086";

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{A Person: {age: 21,name: "John Doe"}}"#);
  let invocable_name = "A Decision Ctx with DT";
  assert_decision(&MODEL_EVALUATOR, NAMESPACE, invocable_name, &ctx, r#""Hello John Doe!""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{A Person: {age: 47,name: "John Doe"}}"#);
  let invocable_name = "A Decision Ctx with DT";
  assert_decision(&MODEL_EVALUATOR, NAMESPACE, invocable_name, &ctx, r#""Respectfully, Hello John Doe!""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(NAMESPACE, invocable_name, &ctx));
}
