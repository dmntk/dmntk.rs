use super::*;
use dmntk_examples::*;

static MODEL_EVALUATOR: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0086_IMPORT, DMN_3_0086]));

const NAMESPACE: &str = "https://dmntk.io/3_0086";

#[test]
fn _0001() {
  let ctx = context(r#" { A Person: { age: 21, name: "John Doe"}} "#);
  assert_decision(&MODEL_EVALUATOR, NAMESPACE, "A Decision Ctx with DT", &ctx, r#""Hello John Doe!""#);
}

#[test]
fn _0002() {
  let ctx = context(r#" { A Person: {age: 47,name: "John Doe"}} "#);
  assert_decision(&MODEL_EVALUATOR, NAMESPACE, "A Decision Ctx with DT", &ctx, r#""Respectfully, Hello John Doe!""#);
}
