mod ascii_model;
mod compatibility;
mod decision_tables;

use std::fs;
use std::fs::File;
use std::io::Write;

/// Name of the target directory.
const TARGET_DIR: &str = "../target/gendoc";

/// Utility function for generating HTML file for decision table defined as text.
fn gen_html_from_model(model: &str, output_file_name: &str) {
  let definitions = dmntk_model::parse(model).expect("parsing model failed");
  let html = crate::dmn_model_to_html(&definitions);
  assert_eq!("<!DOCTYPE html>", &html[0..15]);
  fs::create_dir_all(TARGET_DIR).expect("creating target directories failed");
  let mut file = File::create(format!("{TARGET_DIR}/{output_file_name}.html")).expect("creating output file failed");
  file.write_all(html.as_bytes()).expect("saving output file failed");
}

/// Utility macro for generating HTML file for DMN™ model.
macro_rules! export_model {
  ($t:tt) => {{
    gen_html_from_model(dmntk_examples::$t, stringify!($t));
  }};
}

use export_model;
