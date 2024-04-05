use super::*;
use dmntk_examples::*;

static MODEL_EVALUATOR_A: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_A]));
static MODEL_EVALUATOR_B1: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_B1, DMN_3_0089_MODEL_A]));
static MODEL_EVALUATOR_B2: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_B2, DMN_3_0089_MODEL_A]));
static MODEL_EVALUATOR_C: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_C, DMN_3_0089_MODEL_B1, DMN_3_0089_MODEL_B2, DMN_3_0089_MODEL_A]));

#[test]
fn _0001() {
  let ctx = context(r#" { Person name: "Jenny" } "#);
  assert_decision(
    &MODEL_EVALUATOR_A,
    "http://www.trisotech.com/definitions/_ae5b3c17-1ac3-4e1d-b4f9-2cf861aec6d9",
    "Greet the Person",
    &ctx,
    r#""Hello, Jenny""#,
  );
}

#[test]
fn _0002() {
  let ctx = context(r#" { Person name: "Waldy" } "#);
  assert_decision(
    &MODEL_EVALUATOR_B1,
    "http://www.trisotech.com/definitions/_2a1d771a-a899-4fef-abd6-fc894332337c",
    "Evaluating Say Hello",
    &ctx,
    r#""Evaluating Say Hello to: Hello, Waldy""#,
  );
}

#[test]
fn _0003() {
  let ctx = context(r#" { Model A: { Person name: "John" }} "#);
  assert_decision(
    &MODEL_EVALUATOR_B1,
    "http://www.trisotech.com/definitions/_2a1d771a-a899-4fef-abd6-fc894332337c",
    "Evaluating Say Hello",
    &ctx,
    r#""Evaluating Say Hello to: Hello, John""#,
  );
}

#[test]
fn _0004() {
  let ctx = context(
    r#"{
      Person name: "Johnny", 
      Model A: { Person name: "John" }
    } "#,
  );
  assert_decision(
    &MODEL_EVALUATOR_B1,
    "http://www.trisotech.com/definitions/_2a1d771a-a899-4fef-abd6-fc894332337c",
    "Evaluating Say Hello",
    &ctx,
    r#""Evaluating Say Hello to: Hello, John""#,
  );
}

#[test]
fn _0005() {
  let ctx = context(r#" { Person name: "Cecil" } "#);
  assert_decision(
    &MODEL_EVALUATOR_B2,
    "http://www.trisotech.com/definitions/_9d46ece4-a96c-4cb0-abc0-0ca121ac3768",
    "Evaluating B2 Say Hello",
    &ctx,
    r#""Evaluating Say Hello to: Hello, Cecil""#,
  );
}

#[test]
fn _0006() {
  let ctx = context(r#" { Model A: { Person name: "Peter" }} "#);
  assert_decision(
    &MODEL_EVALUATOR_B2,
    "http://www.trisotech.com/definitions/_9d46ece4-a96c-4cb0-abc0-0ca121ac3768",
    "Evaluating B2 Say Hello",
    &ctx,
    r#""Evaluating Say Hello to: Hello, Peter""#,
  );
}

#[test]
fn _0007() {
  let ctx = context(
    r#"{
       Person name: "Patricia",
       Model A: { Person name: "Peter" }
     }"#,
  );
  assert_decision(
    &MODEL_EVALUATOR_B2,
    "http://www.trisotech.com/definitions/_9d46ece4-a96c-4cb0-abc0-0ca121ac3768",
    "Evaluating B2 Say Hello",
    &ctx,
    r#""Evaluating Say Hello to: Hello, Peter""#,
  );
}

#[test]
fn _0008() {
  let ctx = context(
    r#"{ 
      Model B1: { Model A: { Person name: "Bob" }},
      Model B2: { Model A: { Person name: "John" }} 
    }"#,
  );
  assert_decision(
    &MODEL_EVALUATOR_C,
    "http://www.trisotech.com/definitions/_10435dcd-8774-4575-a338-49dd554a0928",
    "Model C Decision based on Bs",
    &ctx,
    r#""B1: Evaluating Say Hello to: Hello, Bob; B2: Evaluating Say Hello to: Hello, John""#,
  );
}

#[test]
fn _0009() {
  let ctx = context(
    r#"{ 
      Person name: "Janusz Biznesu",
      Model B1: {
        Person name: "Bobby",
        Model A: { Person name: "Bob" }
      },
      Model B2: {
        Person name: "Johnny",
        Model A: { Person name: "John" }
      } 
    }"#,
  );
  assert_decision(
    &MODEL_EVALUATOR_C,
    "http://www.trisotech.com/definitions/_10435dcd-8774-4575-a338-49dd554a0928",
    "Model C Decision based on Bs",
    &ctx,
    r#""B1: Evaluating Say Hello to: Hello, Bob; B2: Evaluating Say Hello to: Hello, John""#,
  );
}
