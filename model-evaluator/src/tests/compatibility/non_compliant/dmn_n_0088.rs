use dmntk_examples::DMN_N_0088;

#[test]
fn _0001() {
  assert_eq!(
    "<ModelValidatorError> cyclic dependency between item definitions",
    dmntk_model::parse(DMN_N_0088).err().unwrap().to_string()
  );
}
