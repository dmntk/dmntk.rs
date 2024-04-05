use dmntk_common::{DmntkError, ToErrorMessage};
use dmntk_feel::FeelType;

/// Errors related to model evaluation.
#[derive(ToErrorMessage)]
struct ModelEvaluatorError(String);

pub fn err_business_knowledge_model_with_reference_not_found(namespace: &str, id: &str) -> DmntkError {
  ModelEvaluatorError(format!("no business knowledge model with reference: '{namespace}#{id}'")).into()
}

pub fn err_empty_literal_expression() -> DmntkError {
  ModelEvaluatorError("empty literal expression".into()).into()
}

pub fn err_empty_encapsulated_logic() -> DmntkError {
  ModelEvaluatorError("empty encapsulated logic in business knowledge model".into()).into()
}

pub fn err_invalid_item_definition_type(s: &str) -> DmntkError {
  ModelEvaluatorError(format!("invalid item definition type for '{s}'")).into()
}

pub fn err_unsupported_feel_type(feel_type: FeelType, s: &str) -> DmntkError {
  ModelEvaluatorError(format!("unsupported FEEL type: {feel_type} in {s}")).into()
}

pub fn err_empty_feel_type() -> DmntkError {
  ModelEvaluatorError("empty FEEL type".into()).into()
}

pub fn err_empty_function_body() -> DmntkError {
  ModelEvaluatorError("empty function definition body".into()).into()
}
