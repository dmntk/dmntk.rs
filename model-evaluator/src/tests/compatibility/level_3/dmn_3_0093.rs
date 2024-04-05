use super::*;

from_examples!(DMN_3_0093);

static_context!(CTX, r#"{}"#);

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "test_001", &CTX, r#"null(invalid @ literal: foo)"#);
}

#[test]
fn _0002() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "date_001", &CTX, r#"true"#);
}
