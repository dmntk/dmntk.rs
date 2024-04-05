#![feature(test)]

extern crate test;

use dmntk_feel::values::Value;
use dmntk_feel::{scope, value_null, value_number, FeelNumber, FeelScope};
use dmntk_feel_evaluator::{prepare, BuildContext};
use test::Bencher;

#[bench]
fn feel_evaluator_addition_0001(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"1+2"#;
  let node = dmntk_feel_parser::parse_expression(&scope, input, false).unwrap();
  let evaluator = prepare(&BuildContext::default(), &node).unwrap();
  assert_eq!(value_number!(3), evaluator(&scope));
  b.iter(|| evaluator(&scope));
}

#[bench]
fn feel_evaluator_addition_0002(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"1+2+3"#;
  let node = dmntk_feel_parser::parse_expression(&scope, input, false).unwrap();
  let evaluator = prepare(&BuildContext::default(), &node).unwrap();
  assert_eq!(value_number!(6), evaluator(&scope));
  b.iter(|| evaluator(&scope));
}

#[bench]
fn feel_evaluator_addition_0003(b: &mut Bencher) {
  let scope = scope!();
  scope.set_value(&"a".into(), value_null!());
  scope.set_value(&"b".into(), value_null!());
  let input = r#"a+b"#;
  let node = dmntk_feel_parser::parse_expression(&scope, input, false).unwrap();
  let evaluator = prepare(&BuildContext::default(), &node).unwrap();
  scope.set_value(&"a".into(), value_number!(1838, 2));
  scope.set_value(&"b".into(), value_number!(162, 2));
  assert_eq!(value_number!(20), evaluator(&scope));
  b.iter(|| evaluator(&scope));
}
