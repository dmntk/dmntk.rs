#![feature(test)]

extern crate test;

use dmntk_feel::values::Value;
use dmntk_feel::{scope, value_number, FeelScope};
use dmntk_feel_evaluator::{prepare, BuildContext};
use test::Bencher;

#[bench]
fn feel_evaluator_bif_abs_0001(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"abs(-1)"#;
  let node = dmntk_feel_parser::parse_textual_expression(&scope, input, false).unwrap();
  let evaluator = prepare(&BuildContext::default(), &node).unwrap();
  assert_eq!(value_number!(1), evaluator(&scope));
  b.iter(|| evaluator(&scope));
}
