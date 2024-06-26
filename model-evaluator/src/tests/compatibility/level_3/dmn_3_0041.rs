use super::*;

from_examples!(DMN_3_0041);

#[test]
fn _0001() {
  let ctx = context(r#"{Principal: 600000,Term: 360}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Boxed Context", &ctx, r#"2778.693549432766768088520383236288"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{Principal: 30000,Term: 60}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Boxed Context", &ctx, r#"549.1175498364002934927000148859422"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{Principal: 600000,Term: 365}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "Boxed Context", &ctx, r#"2758.11609989659140087141889328903"#);
}
