use super::super::*;

const DMN_0003: &str = include_str!("_0003.dmn");

model_evaluator!(DMN_0003);

const NAMESPACE: &str = "https://dmntk.io/services";

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, NAMESPACE, "input", &ctx, r#"27"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, NAMESPACE, "output", &ctx, r#"27"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, NAMESPACE, "ds", &ctx, r#"27"#);
}
