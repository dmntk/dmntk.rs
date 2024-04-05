use crate::builders::build_evaluator;
use crate::BuildContext;
use dmntk_feel::values::Value;
use dmntk_feel::{FeelNumber, FeelScope};
use dmntk_feel_parser::AstNode;
use dmntk_feel_temporal::{Day, FeelDate, FeelDateTime, FeelDaysAndTimeDuration, FeelTime, FeelYearsAndMonthsDuration, Month, Year};

mod addition;
mod arithmetic_negation;
mod bifs;
mod comments;
mod comparison_between;
mod comparison_eq;
mod comparison_ge;
mod comparison_gt;
mod comparison_in;
mod comparison_le;
mod comparison_lt;
mod comparison_ne;
mod conjunction;
mod context;
mod disjunction;
mod division;
mod empty_input;
mod every_expression;
mod exponentiation;
mod external_functions;
mod filter;
mod for_expression;
mod formal_parameters;
mod function_definition;
mod function_invocation;
mod if_expression;
mod instance_of;
mod iterations;
mod join;
mod literal_at;
mod literal_boolean;
mod literal_numeric;
mod multiline;
mod multiplication;
mod name;
mod negation;
mod out_operator;
mod parentheses;
mod path;
mod properties;
mod range;
mod satisfies;
mod some_expression;
mod subtraction;
mod types;
mod unary_tests;
mod various;

const SECONDS_IN_DAY: i64 = 86_400;
const SECONDS_IN_HOUR: i64 = 3_600;
const SECONDS_IN_MINUTE: i64 = 60;

/// Utility function that tests evaluation of boolean value.
pub fn te_bool(trace: bool, scope: &FeelScope, s: &str, expected: bool) {
  textual_expression(trace, scope, s, Value::Boolean(expected));
}

/// Utility function that tests evaluation of date value.
pub fn te_date(trace: bool, scope: &FeelScope, s: &str, year: Year, month: Month, day: Day) {
  textual_expression(trace, scope, s, Value::Date(FeelDate::new(year, month, day)));
}

/// Utility function that tests evaluation of local date and time value.
pub fn te_date_time_local(trace: bool, scope: &FeelScope, s: &str, date: (Year, Month, Day), time: (u8, u8, u8, u64)) {
  let (year, month, day) = date;
  let (hour, min, sec, nano) = time;
  textual_expression(trace, scope, s, Value::DateTime(FeelDateTime::local(year, month, day, hour, min, sec, nano)));
}

/// Utility function that tests evaluation of UTC date and time value.
pub fn te_date_time_utc(trace: bool, scope: &FeelScope, s: &str, date: (Year, Month, Day), time: (u8, u8, u8, u64)) {
  let (year, month, day) = date;
  let (hour, min, sec, nano) = time;
  textual_expression(trace, scope, s, Value::DateTime(FeelDateTime::utc(year, month, day, hour, min, sec, nano)));
}

/// Utility function that tests evaluation of date and time value with explicit offset.
pub fn te_date_time_offset(trace: bool, scope: &FeelScope, s: &str, date: (Year, Month, Day), time: (u8, u8, u8, u64), offset: i32) {
  textual_expression(trace, scope, s, Value::DateTime(FeelDateTime::offset(date, time, offset)));
}

/// Utility function that creates scope from specified input.
pub fn te_scope(input: &str) -> FeelScope {
  let scope = FeelScope::default();
  match dmntk_feel_parser::parse_context(&scope, input, false) {
    Ok(node) => match crate::evaluate(&scope, &node) {
      Ok(value) => match value {
        Value::Context(ctx) => ctx.into(),
        _ => {
          println!("ERROR (INVALID VALUE TYPE): {value}");
          panic!("te_scope failed");
        }
      },
      Err(reason) => {
        println!("{reason}");
        panic!("te_scope failed");
      }
    },
    Err(reason) => {
      println!("ERROR (REASON): {reason}");
      panic!("te_scope failed");
    }
  }
}

/// Utility function that tests evaluation of numeric values.
pub fn te_number(trace: bool, scope: &FeelScope, s: &str, num: i64, scale: i32) {
  textual_expression(trace, scope, s, Value::Number(FeelNumber::new(num, scale)));
}

/// Utility function that tests evaluation of numeric values.
pub fn te_number_x(trace: bool, scope: &FeelScope, s: &str, num: &str) {
  textual_expression(trace, scope, s, Value::Number(num.parse::<FeelNumber>().unwrap()));
}

/// Utility function that tests evaluation to null value.
fn te_null(trace: bool, scope: &FeelScope, s: &str, t: &str) {
  textual_expression(trace, scope, s, if t.is_empty() { Value::Null(None) } else { Value::Null(Some(t.to_owned())) });
}

/// Utility function that tests evaluation to an error result.
pub fn te_none(trace: bool, scope: &FeelScope, s: &str) {
  assert!(dmntk_feel_parser::parse_textual_expression(scope, s, trace).is_err());
}

/// Utility function that tests evaluation to string value.
fn te_string(trace: bool, scope: &FeelScope, s: &str, expected: &str) {
  textual_expression(trace, scope, s, Value::String(expected.to_string()));
}

/// Utility function that tests evaluation of year and months duration.
pub fn te_years_and_months_duration(trace: bool, scope: &FeelScope, s: &str, years: i64, months: i64) {
  textual_expression(trace, scope, s, Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::from_ym(years, months)));
}

/// Utility function that tests evaluation of year and months duration.
pub fn te_years_and_months_duration_x(trace: bool, scope: &FeelScope, s: &str, expected: &str) {
  textual_expression(trace, scope, s, Value::YearsAndMonthsDuration(expected.try_into().unwrap()));
}

/// Utility function that tests evaluation of days and time duration.
pub fn te_days_and_time_duration(trace: bool, scope: &FeelScope, s: &str, neg: bool, sec: i64, nano: i64) {
  textual_expression(
    trace,
    scope,
    s,
    Value::DaysAndTimeDuration(if neg {
      -FeelDaysAndTimeDuration::from_sn(sec, nano)
    } else {
      FeelDaysAndTimeDuration::from_sn(sec, nano)
    }),
  );
}

/// Utility function that tests evaluation of days and time duration.
pub fn te_days_and_time_duration_x(trace: bool, scope: &FeelScope, s: &str, expected: &str) {
  textual_expression(trace, scope, s, Value::DaysAndTimeDuration(expected.try_into().unwrap()));
}

/// Utility function that tests evaluation of time.
pub fn te_time(trace: bool, scope: &FeelScope, s: &str, expected: FeelTime) {
  textual_expression(trace, scope, s, Value::Time(expected));
}

/// Utility function that tests evaluation to specified value.
pub fn te_value(trace: bool, scope: &FeelScope, actual: &str, expected: &str) {
  match dmntk_feel_parser::parse_textual_expression(scope, expected, trace) {
    Ok(node) => match build_evaluator(&BuildContext::default(), &node) {
      Ok(evaluator) => textual_expression(trace, scope, actual, evaluator(scope)),
      Err(reason) => {
        println!("{reason}");
        panic!("te_value failed");
      }
    },
    Err(reason) => {
      println!("ERROR: {reason}");
      panic!("te_value failed");
    }
  }
}

/// Utility function that tests evaluation to specified value represented by boxed expression.
pub fn te_be_value(trace: bool, scope: &FeelScope, actual: &str, expected: &str) {
  match dmntk_feel_parser::parse_expression(scope, expected, false) {
    Ok(node) => match crate::evaluate(scope, &node) {
      Ok(value) => textual_expression(trace, scope, actual, value),
      Err(reason) => {
        println!("{reason}");
        panic!("te_value failed");
      }
    },
    Err(reason) => {
      println!("ERROR (REASON): {reason}");
      panic!("te_value failed");
    }
  }
}

/// Utility function that tests evaluation to specified value represented by boxed expression.
pub fn be_be_value(trace: bool, scope: &FeelScope, actual: &str, expected: &str) {
  match dmntk_feel_parser::parse_boxed_expression(scope, expected, trace) {
    Ok(node) => match crate::evaluate(scope, &node) {
      Ok(value) => boxed_expression(trace, scope, actual, value),
      Err(reason) => {
        println!("{reason}");
        panic!("te_value failed");
      }
    },
    Err(reason) => {
      println!("ERROR (REASON): {reason}");
      panic!("te_value failed");
    }
  }
}

/// Utility function that takes a text parameter, evaluates the boxed expression
/// represented by this text and compares the result with provided expected value.
/// The result must be equal to expected value, otherwise an error is reported.
pub fn boxed_expression(trace: bool, scope: &FeelScope, text: &str, expected: Value) {
  match dmntk_feel_parser::parse_boxed_expression(scope, text, trace) {
    Ok(node) => match crate::evaluate(scope, &node) {
      Ok(value) => assert_eq!(value, expected),
      Err(reason) => {
        println!("{reason}");
        panic!("boxed_expression failed");
      }
    },
    Err(reason) => {
      println!("ERROR: {reason}");
      panic!("boxed_expression failed");
    }
  }
}

/// Utility function that takes a text parameter, evaluates the textual expression
/// represented by this text and compares the result with provided expected value.
/// The result must be equal to expected value, otherwise an error is reported.
fn textual_expression(trace: bool, scope: &FeelScope, text: &str, expected: Value) {
  match dmntk_feel_parser::parse_textual_expression(scope, text, trace) {
    Ok(node) => match build_evaluator(&BuildContext::default(), &node) {
      Ok(evaluator) => {
        let actual = evaluator(scope);
        assert_eq!(actual, expected, "ERROR\nexpected: {expected}\n  actual: {actual}\n");
      }
      Err(reason) => {
        panic!("building evaluator for textual expression failed with reason: {}", reason);
      }
    },
    Err(reason) => {
      panic!("parsing textual expression failed with reason: {}", reason);
    }
  }
}

/// Utility function that checks if unary tests are correctly parsed.
pub fn valid_unary_tests(trace: bool, scope: &FeelScope, text: &str) {
  match dmntk_feel_parser::parse_unary_tests(scope, text, trace) {
    Ok(node) => match build_evaluator(&BuildContext::default(), &node) {
      Ok(evaluator) => {
        if let v @ Value::Null(_) = evaluator(scope) {
          panic!("evaluating unary tests failed, value: {}", v)
        }
      }
      Err(reason) => {
        panic!("building evaluator for unary tests failed with reason: {}", reason);
      }
    },
    Err(reason) => {
      panic!("parsing unary tests failed with reason: {}", reason);
    }
  }
}

pub fn satisfies(trace: bool, scope: &FeelScope, input_expression: &str, input_values: &str, input_entry: &str, expected: bool) {
  let input_expression_node = dmntk_feel_parser::parse_textual_expression(scope, input_expression, trace).unwrap();
  let input_entry_node = dmntk_feel_parser::parse_unary_tests(scope, input_entry, trace).unwrap();
  let node = if !input_values.is_empty() {
    let input_values_node = dmntk_feel_parser::parse_unary_tests(scope, input_values, trace).unwrap();
    let left = AstNode::In(Box::new(input_expression_node.clone()), Box::new(input_values_node));
    let right = AstNode::In(Box::new(input_expression_node), Box::new(input_entry_node));
    AstNode::And(Box::new(left), Box::new(right))
  } else {
    AstNode::In(Box::new(input_expression_node), Box::new(input_entry_node))
  };
  match crate::evaluate(scope, &node) {
    Ok(value) => assert_eq!(value, Value::Boolean(expected)),
    Err(reason) => {
      println!("ERROR: {reason}");
      panic!("`satisfies` failed");
    }
  }
}

pub fn satisfies_null(trace: bool, scope: &FeelScope, input_expression: &str, input_values: &str, input_entry: &str, expected: &str) {
  let input_expression_node = dmntk_feel_parser::parse_textual_expression(scope, input_expression, trace).unwrap();
  let input_entry_node = dmntk_feel_parser::parse_unary_tests(scope, input_entry, trace).unwrap();
  let node = if !input_values.is_empty() {
    let input_values_node = dmntk_feel_parser::parse_unary_tests(scope, input_values, trace).unwrap();
    let left = AstNode::In(Box::new(input_expression_node.clone()), Box::new(input_values_node));
    let right = AstNode::In(Box::new(input_expression_node), Box::new(input_entry_node));
    AstNode::And(Box::new(left), Box::new(right))
  } else {
    AstNode::In(Box::new(input_expression_node), Box::new(input_entry_node))
  };
  match crate::evaluate(scope, &node) {
    Ok(value) => assert_eq!(value, Value::Null(Some(expected.to_string()))),
    Err(reason) => {
      println!("ERROR: {reason}");
      panic!("`satisfies` failed");
    }
  }
}
