use super::*;

from_examples!(DMN_3_0076);

static_context!(CTX, r#"{}"#);

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "boxed_001", &CTX, r#"456"#);
}

#[test]
fn _0002() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "incorrect_001",
    &CTX,
    r#"null(java.lang.NoSuchMethodException: java.lang.Math.foo(double))"#,
  );
}

#[test]
fn _0003() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "incorrect_002",
    &CTX,
    r#"null(java.lang.ClassNotFoundException: java.lang.Foo)"#,
  );
}

#[test]
fn _0004() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "incorrect_003",
    &CTX,
    r#"null(java.lang.NoSuchMethodException: java.lang.Math.max(java.lang.String,java.lang.String))"#,
  );
}

#[test]
fn _0005() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "literal_001", &CTX, r#"-0.8879689066918555"#);
}

#[test]
fn _0006() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "literal_002", &CTX, r#"456.78"#);
}

#[test]
fn _0007() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "literal_003", &CTX, r#"456"#);
}

#[test]
fn _0008() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "literal_004", &CTX, r#"456"#);
}

#[test]
fn _0009() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "literal_005", &CTX, r#"123"#);
}

#[test]
fn _0010() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "literal_006", &CTX, r#"3"#);
}

#[test]
fn _0011() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "literal_007", &CTX, r#""a""#);
}

#[test]
fn _0012() {
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "literal_007_a",
    &CTX,
    r#"null(simple DTO conversion to object failed, class: char, type: XSD_STRING)"#,
  );
}

#[test]
fn _0013() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "literal_008", &CTX, r#"456"#);
}

#[test]
fn _0014() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "literal_009", &CTX, r#"456.78"#);
}

#[test]
fn _0015() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "literal_010", &CTX, r#"123"#);
}

#[test]
fn _0016() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "literal_011", &CTX, r#"1234.56"#);
}

#[test]
fn _0017() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "literal_012", &CTX, r#"1234.56"#);
}

#[test]
fn _0018() {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "varargs_001", &CTX, r#""foo bar""#);
}
