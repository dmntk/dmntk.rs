//! # Compatibility benchmarks

use dmntk_examples::*;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::FeelScope;
use dmntk_model_evaluator::ModelEvaluator;
use once_cell::sync::Lazy;
use std::sync::Arc;
use test::Bencher;

mod level_2;
mod level_3;
mod non_compliant;

macro_rules! from_examples {
  ($model_name:tt) => {
    model_evaluator_from_examples!($model_name);
    model_namespace_from_examples!($model_name);
  };
}

macro_rules! model_evaluator_from_examples {
  ($model_name:tt) => {
    static MODEL_EVALUATOR: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluator(dmntk_examples::$model_name));
  };
}

macro_rules! model_namespace_from_examples {
  ($model_name:tt) => {
    static MODEL_NAMESPACE: Lazy<String> = Lazy::new(|| build_model_namespace(dmntk_examples::$model_name));
  };
}

use dmntk_feel_evaluator::BuildContext;
use {from_examples, model_evaluator_from_examples, model_namespace_from_examples};

/// Utility function that builds a model evaluator from a single DMN model.
fn build_model_evaluator(model_content: &str) -> Arc<ModelEvaluator> {
  let definitions = dmntk_model::parse(model_content).unwrap();
  ModelEvaluator::new(&[definitions]).unwrap()
}

/// Utility function that builds a model evaluator from multiple DMN models.
fn build_model_evaluators(model_content: &[&str]) -> Arc<ModelEvaluator> {
  let mut definitions = vec![];
  for content in model_content {
    definitions.push(dmntk_model::parse(content).unwrap());
  }
  ModelEvaluator::new(&definitions).unwrap()
}

/// Utility function that returns a namespace from a single DMN model.
fn build_model_namespace(model_content: &str) -> String {
  let definitions = dmntk_model::parse(model_content).unwrap();
  definitions.namespace().to_string()
}

/// Utility function that creates a FEEL context from specified input expression.
pub fn context(input: &str) -> FeelContext {
  let scope = FeelScope::default();
  match dmntk_feel_parser::parse_context(&scope, input, false) {
    Ok(node) => match dmntk_feel_evaluator::prepare(&BuildContext::default(), &node) {
      Ok(evaluator) => match evaluator(&scope) {
        Value::Context(ctx) => ctx,
        other => panic!("ERROR: expected context value, actual value is: {}", other as Value),
      },
      Err(reason) => panic!("ERROR: building evaluator failed with reason: {reason}"),
    },
    Err(reason) => panic!("ERROR: parsing context failed with reason: {reason}"),
  }
}

/// Utility function that evaluates a `Decision` specified by name and compares the result.
fn assert_decision(model_evaluator: &ModelEvaluator, namespace: &str, invocable_name: &str, input_data: &FeelContext, expected: &str) {
  let actual = model_evaluator.evaluate_invocable(namespace, invocable_name, input_data).to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the decision does not match the expected value:\n  expected: {expected}\n    actual: {actual}\n"
  );
}

/// Utility function that evaluates a `BusinessKnowledgeModel` specified by name and compares the result.
fn assert_business_knowledge_model(model_evaluator: &ModelEvaluator, namespace: &str, invocable_name: &str, input_data: &FeelContext, expected: &str) {
  let actual = model_evaluator.evaluate_invocable(namespace, invocable_name, input_data).to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the business knowledge model does not match the expected value:\n  expected: {expected}\n    actual: {actual}\n"
  );
}

/// Utility function that evaluates a `DecisionService` specified by name and compares the result with expected value.
fn assert_decision_service(model_evaluator: &ModelEvaluator, namespace: &str, invocable_name: &str, input: &str, expected: &str) {
  let actual = model_evaluator.evaluate_invocable(namespace, invocable_name, &context(input)).to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the decision service does not match the expected value:\n  expected: {expected}\n    actual: {actual}\n"
  );
}
