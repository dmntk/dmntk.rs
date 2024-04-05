use dmntk_common::{DmntkError, ToErrorMessage};

/// `FEEL` expressions evaluator errors.
#[derive(ToErrorMessage)]
struct FeelEvaluatorError(String);

pub fn err_not_a_context() -> DmntkError {
  FeelEvaluatorError("expected FEEL context as an input".to_string()).into()
}

pub fn err_expected_positional_or_named_parameter() -> DmntkError {
  FeelEvaluatorError("expected positional or named parameter".to_string()).into()
}

pub fn err_expected_ast_node_parameter_name(s: &str) -> DmntkError {
  FeelEvaluatorError(format!("expected AstNode::ParameterName, actual node is {s}")).into()
}

pub fn err_expected_ast_node(expected: &str, actual: &str) -> DmntkError {
  FeelEvaluatorError(format!("expected AST node {expected}, actual AST node is {actual}")).into()
}

pub fn err_unexpected_ast_node(s: &str) -> DmntkError {
  FeelEvaluatorError(format!("unexpected AST node in evaluator builder {s}")).into()
}
