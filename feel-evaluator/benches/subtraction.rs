#![feature(test)]

extern crate test;

use dmntk_feel::values::Value;
use dmntk_feel::{scope, value_null, value_number, FeelScope};
use dmntk_feel_evaluator::{prepare, BuildContext};
use test::Bencher;

#[bench]
fn feel_evaluator_subtraction_0001(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"2-1"#;
  let node = dmntk_feel_parser::parse_textual_expression(&scope, input, false).unwrap();
  let evaluator = prepare(&BuildContext::default(), &node).unwrap();
  assert_eq!(value_number!(1), evaluator(&scope));
  b.iter(|| evaluator(&scope));
}

#[bench]
fn feel_evaluator_subtraction_0002(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"5-2-1"#;
  let node = dmntk_feel_parser::parse_textual_expression(&scope, input, false).unwrap();
  let evaluator = prepare(&BuildContext::default(), &node).unwrap();
  assert_eq!(value_number!(2), evaluator(&scope));
  b.iter(|| evaluator(&scope));
}

#[bench]
fn feel_evaluator_subtraction_0003(b: &mut Bencher) {
  let scope = scope!();
  scope.set_value(&"a".into(), value_null!());
  scope.set_value(&"b".into(), value_null!());
  let input = r#"a-b"#;
  let node = dmntk_feel_parser::parse_textual_expression(&scope, input, false).unwrap();
  let evaluator = prepare(&BuildContext::default(), &node).unwrap();
  scope.set_value(&"a".into(), value_number!(18));
  scope.set_value(&"b".into(), value_number!(3));
  assert_eq!(value_number!(15), evaluator(&scope));
  b.iter(|| evaluator(&scope));
}
