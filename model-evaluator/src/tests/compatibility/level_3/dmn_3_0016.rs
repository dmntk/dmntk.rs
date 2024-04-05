use super::*;

from_examples!(DMN_3_0016);

#[test]
fn _0001() {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "priceTable1",
    &ctx,
    r#"[{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]"#,
  );
}

#[test]
fn _0002() {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "everyGtTen1", &ctx, r#"false"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "everyGtTen2", &ctx, r#"false"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "someGtTen1", &ctx, r#"true"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "someGtTen2", &ctx, r#"true"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "everyGtTen3", &ctx, r#"false"#);
}
