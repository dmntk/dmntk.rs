use super::*;

from_examples!(DMN_3_0016);

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  let invocable_name = "priceTable1";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"[{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  let invocable_name = "everyGtTen1";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  let invocable_name = "everyGtTen2";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  let invocable_name = "someGtTen1";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  let invocable_name = "someGtTen2";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"true"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  let invocable_name = "everyGtTen3";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"false"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, invocable_name, &ctx));
}
