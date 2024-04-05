use super::*;

from_examples!(DMN_3_0012);

#[test]
fn _0001() {
  let ctx = context(r#"{list1: ["a","b","c"],list2: ["x","y","z"]}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "listContainsList", &ctx, r#"false"#);
}
