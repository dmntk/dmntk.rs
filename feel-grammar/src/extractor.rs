//! Tool for extracting `LALR` parsing tables from the output generated
//! by `Bison` parser generator for `C` language.

use convert_case::{Case, Casing};
use regex::Regex;
use std::collections::HashSet;
use std::fmt::Write as _;

const HEADER: &str = r#"
//! Parsing tables extracted from files generated by `Bison` for `C` language.
//! This file was generated by dedicated tool, do not modify manually.
"#;

const USE_CLAUSE: &str = r#"
use dmntk_common::Result;
"#;

/// Extracts parsing tables and generates the output code.
pub fn extract(input: &str) -> String {
  let token_types = extract_token_types(input);
  let symbol_kinds = extract_symbol_kinds(input);
  let yy_pact_n_inf = extract_define_paren(input, "YYPACT_NINF");
  let yy_table_n_inf = extract_define_paren(input, "YYTABLE_NINF");
  let yy_final = extract_define(input, "YYFINAL");
  let yy_last = extract_define(input, "YYLAST");
  let yy_n_tokens = extract_define(input, "YYNTOKENS");
  let yy_translate = extract_numeric_table(input, "yytranslate");
  let yy_pact = extract_numeric_table(input, "yypact");
  let yy_def_act = extract_numeric_table(input, "yydefact");
  let yy_p_goto = extract_numeric_table(input, "yypgoto");
  let yy_def_goto = extract_numeric_table(input, "yydefgoto");
  let yy_table = extract_numeric_table(input, "yytable");
  let yy_check = extract_numeric_table(input, "yycheck");
  let yy_r1 = extract_numeric_table(input, "yyr1");
  let yy_r2 = extract_numeric_table(input, "yyr2");
  let actions = extract_semantic_actions(input);
  // generate output file content
  let mut output = String::with_capacity(10_000);
  // generate the file header
  {
    output.push_str(HEADER);
  }
  // generate the use clause
  {
    output.push_str(USE_CLAUSE);
  }
  // generate token types
  {
    output.push_str("\n/// Types of tokens returned by lexer.\n");
    output.push_str("#[derive(Clone)]\n");
    output.push_str("pub enum TokenType {\n");
    for (name, value) in &token_types {
      if !name.starts_with("Prec") {
        let _ = writeln!(output, "  {name} = {value},");
      }
    }
    output.push_str("}\n")
  }
  // generate symbol kinds
  {
    output.push_str("\n/// Kinds of symbols recognized by parser.\n");
    output.push_str("#[allow(clippy::enum_variant_names)]\n");
    output.push_str("pub enum SymbolKind {\n");
    for (name, value) in &symbol_kinds {
      if name.starts_with("Yy") {
        let _ = writeln!(output, "  {name} = {value},");
      }
    }
    output.push_str("}\n")
  }
  {
    // generate constants
    output.push_str("\n/// YY_PACT_N_INF.\n");
    let _ = writeln!(output, "pub const YY_PACT_N_INF: i16 = {yy_pact_n_inf};");
    output.push_str("\n/// YY_TABLE_N_INF.\n");
    let _ = writeln!(output, "pub const YY_TABLE_N_INF: i16 = {yy_table_n_inf};");
    output.push_str("\n/// YY_FINAL.\n");
    let _ = writeln!(output, "pub const YY_FINAL: usize = {yy_final};");
    output.push_str("\n/// YY_LAST.\n");
    let _ = writeln!(output, "pub const YY_LAST: i16 = {yy_last};");
    output.push_str("\n/// YY_N_TOKENS.\n");
    let _ = writeln!(output, "pub const YY_N_TOKENS: usize = {yy_n_tokens};");
  }
  // generate tables
  {
    // YY_TRANSLATE
    output.push_str("\n/// `YY_TRANSLATE[TOKEN-NUM]` - symbol number corresponding to TOKEN-NUM as returned by lexer.\n");
    let _ = writeln!(output, "pub const YY_TRANSLATE: [{}; {}] = [", yy_translate.0, yy_translate.1.len());
    for value in &yy_translate.1 {
      let _ = write!(output, "{value}, ");
    }
    output.push_str("\n];\n");
    // YY_PACT
    output.push_str("\n/// `YY_PACT[STATE-NUM]` - index in YY_TABLE of the portion describing STATE-NUM.\n");
    let _ = writeln!(output, "pub const YY_PACT: [{}; {}] = [", yy_pact.0, yy_pact.1.len());
    for value in &yy_pact.1 {
      let _ = write!(output, "{value}, ");
    }
    output.push_str("\n];\n");
    // YY_DEF_ACT
    output.push_str("\n/// `YY_DEF_ACT[STATE-NUM]` - default reduction number in state STATE-NUM.\n");
    output.push_str("/// Performed when YY_TABLE does not specify something else to do.\n");
    output.push_str("/// Zero means the default is an error.\n");
    let _ = writeln!(output, "pub const YY_DEF_ACT: [{}; {}] = [", yy_def_act.0, yy_def_act.1.len());
    for value in &yy_def_act.1 {
      let _ = write!(output, "{value}, ");
    }
    output.push_str("\n];\n");
    // YY_P_GOTO
    output.push_str("\n/// `YY_P_GOTO[NTERM-NUM]`\n");
    let _ = writeln!(output, "pub const YY_P_GOTO: [{}; {}] = [", yy_p_goto.0, yy_p_goto.1.len());
    for value in &yy_p_goto.1 {
      let _ = write!(output, "{value}, ");
    }
    output.push_str("\n];\n");
    // YY_DEF_GOTO
    output.push_str("\n/// `YY_DEF_GOTO[NTERM-NUM]`\n");
    let _ = writeln!(output, "pub const YY_DEF_GOTO: [{}; {}] = [", yy_def_goto.0, yy_def_goto.1.len());
    for value in &yy_def_goto.1 {
      let _ = write!(output, "{value}, ");
    }
    output.push_str("\n];\n");
    // YY_TABLE
    output.push_str("\n/// `YY_TABLE[YY_PACT[STATE-NUM]]` - what to do in state STATE-NUM.\n");
    output.push_str("/// If positive, shift that token.\n");
    output.push_str("/// If negative, reduce the rule whose number is the opposite.\n");
    output.push_str("/// If `YY_TABLE_N_INF`, syntax error.\n");
    let _ = writeln!(output, "pub const YY_TABLE: [{}; {}] = [", yy_table.0, yy_table.1.len());
    for value in &yy_table.1 {
      let _ = write!(output, "{value}, ");
    }
    output.push_str("\n];\n");
    // YY_CHECK
    output.push_str("\n/// ???\n");
    let _ = writeln!(output, "pub const YY_CHECK: [{}; {}] = [", yy_check.0, yy_check.1.len());
    for value in &yy_check.1 {
      let _ = write!(output, "{value}, ");
    }
    output.push_str("\n];\n");
    // YY_R1
    output.push_str("\n/// `YY_R1[YYN]` - symbol number of symbol that rule YYN derives.\n");
    let _ = writeln!(output, "pub const YY_R1: [{}; {}] = [", yy_r1.0, yy_r1.1.len());
    for value in &yy_r1.1 {
      let _ = write!(output, "{value}, ");
    }
    output.push_str("\n];\n");
    // YY_R2
    output.push_str("\n/// `YY_R2[YYN]` - number of symbols on the right hand side of rule YYN.\n");
    let _ = writeln!(output, "pub const YY_R2: [{}; {}] = [", yy_r2.0, yy_r2.1.len());
    for value in &yy_r2.1 {
      let _ = write!(output, "{value}, ");
    }
    output.push_str("\n];\n");
  }
  // generate reduce actions trait
  {
    output.push_str("\n///Trait for reduce action definitions.\n");
    output.push_str("pub trait ReduceActions {\n");
    let rule_names = actions.iter().map(|(_, _, name)| name.clone()).collect::<HashSet<String>>();
    let mut sorted_rule_names = rule_names.iter().collect::<Vec<&String>>();
    sorted_rule_names.sort();
    for rule_name in sorted_rule_names {
      let _ = writeln!(output, "  fn action_{rule_name}(&mut self) -> Result<()>;");
    }
    output.push_str("}\n");
  }
  // generate actions
  {
    output.push_str("\n/// Calls requested reduce action.\n");
    output.push_str("pub fn reduce(reduce_actions: &mut impl ReduceActions, rule_number: i16) -> Result<()> {\n");
    output.push_str("  match rule_number {\n");
    for (num, comment, rule) in &actions {
      let _ = writeln!(output, "    {num} => reduce_actions.action_{rule}(), // {comment}");
    }
    output.push_str("    _ => Ok(())\n");
    output.push_str("  }\n");
    output.push_str("}\n");
  }
  output
}

fn extract_token_types(input: &str) -> Vec<(String, i64)> {
  let mut token_types = vec![];
  let re_token_types: Regex = Regex::new("(?m).*enum\\s+yytokentype[^{]+\\{(?P<items>[^}]+)};.*").unwrap();
  let re_token_type: Regex = Regex::new("TOKEN_(?P<name>[A-Za-z_]+)\\s+=\\s+(?P<value>-?[0-9]+)").unwrap();
  if let Some(token_types_captures) = re_token_types.captures(input) {
    if let Some(token_types_match) = token_types_captures.name("items") {
      for line in token_types_match.as_str().to_string().lines() {
        if let Some(token_type_capture) = re_token_type.captures(line.trim()) {
          if let Some(name_match) = token_type_capture.name("name") {
            if let Some(value_match) = token_type_capture.name("value") {
              let value = value_match.as_str().parse::<i64>().unwrap();
              let name = name_match.as_str().to_string().to_uppercase().replace("YY", "YY_").to_case(Case::UpperCamel);
              token_types.push((name, value));
            }
          }
        }
      }
    }
  }
  assert!(!token_types.is_empty(), "no token types found");
  token_types
}

fn extract_symbol_kinds(input: &str) -> Vec<(String, i64)> {
  let mut symbol_kinds = vec![];
  let re_symbol_kinds: Regex = Regex::new("(?m).*enum\\s+yysymbol_kind_t[^{]+\\{(?P<items>[^}]+)};.*").unwrap();
  let re_symbol_kind: Regex = Regex::new("SYMBOL_(?P<name>[A-Za-z_]+)\\s+=\\s+(?P<value>-?[0-9]+)").unwrap();
  if let Some(symbol_kinds_captures) = re_symbol_kinds.captures(input) {
    if let Some(symbol_kinds_match) = symbol_kinds_captures.name("items") {
      for line in symbol_kinds_match.as_str().to_string().lines() {
        if let Some(symbol_kind_capture) = re_symbol_kind.captures(line.trim()) {
          if let Some(name_match) = symbol_kind_capture.name("name") {
            if let Some(value_match) = symbol_kind_capture.name("value") {
              let value = value_match.as_str().parse::<i64>().unwrap();
              let symbol_name = name_match.as_str().to_string();
              let updated_name = if symbol_name.to_lowercase() == symbol_name {
                format!("LHS_{symbol_name}").to_string()
              } else {
                symbol_name
              };
              let name = updated_name.as_str().to_string().to_uppercase().replace("YY", "YY_").to_case(Case::UpperCamel);
              symbol_kinds.push((name, value));
            }
          }
        }
      }
    }
  }
  assert!(!symbol_kinds.is_empty(), "no symbol kinds found");
  symbol_kinds
}

fn extract_define_paren(input: &str, name: &str) -> i64 {
  let pattern = format!(".*#define\\s+{name}\\s+\\((?P<value>-?[0-9]+)\\)");
  extract_value(input, &pattern)
}

fn extract_define(input: &str, name: &str) -> i64 {
  let pattern = format!(".*#define\\s+{name}\\s+(?P<value>-?[0-9]+)");
  extract_value(input, &pattern)
}

fn extract_value(input: &str, pattern: &str) -> i64 {
  let re: Regex = Regex::new(pattern).unwrap();
  if let Some(captures) = re.captures(input) {
    if let Some(value_match) = captures.name("value") {
      let value = value_match.as_str().parse::<i64>().unwrap();
      return value;
    }
  }
  panic!("no value found");
}

fn extract_numeric_table(input: &str, table_name: &str) -> (String, Vec<i64>) {
  let pattern = format!("(?m).*static\\s+const\\s+(?P<type>[a-zA-Z_0-9]+)\\s+{table_name}\\[\\]\\s+=[^{{]+\\{{(?P<values>[^}}]+)\\}};");
  let re: Regex = Regex::new(&pattern).unwrap();
  if let Some(captures) = re.captures(input) {
    let type_match = captures.name("type").unwrap(); // unwrap is ok, `type` group always exists
    let table_type = match type_match.as_str() {
      "yytype_int8" => "i8",
      "yytype_uint8" => "u8",
      "yytype_int16" => "i16",
      _ => {
        panic!("unhandled type for table elements `{}`", type_match.as_str());
      }
    }
    .to_string();
    let values_match = captures.name("values").unwrap(); // unwrap is ok, `values` group always exists
    let s = values_match.as_str().split(',');
    return (table_type, s.map(|a| a.trim().parse::<i64>().unwrap()).collect());
  }
  panic!("no table elements found for table `{table_name}`");
}

fn extract_semantic_actions(input: &str) -> Vec<(i64, String, String)> {
  let mut actions = vec![];
  let modified_input = input.replace("{/*", "#").replace("*/}", "@");
  let re_actions: Regex = Regex::new("(?m).*\\s+switch\\s+\\(yyn\\)[^{]+\\{(?P<actions>[^}]+)}.*").unwrap();
  let re_action: Regex = Regex::new("(?m)case\\s+[0-9]+:\\s+/\\*[^*]+\\*/[^;]+").unwrap();
  let re: Regex = Regex::new("(?m)case\\s+(?P<num>[0-9]+):\\s+/\\*(?P<comment>[^*]+)\\*/[^#]+#(?P<rule>[^@]+)").unwrap();
  if let Some(actions_captures) = re_actions.captures(&modified_input) {
    if let Some(actions_match) = actions_captures.name("actions") {
      for action_match in re_action.find_iter(actions_match.as_str()) {
        if let Some(captures) = re.captures(action_match.as_str()) {
          if let Some(num_match) = captures.name("num") {
            let num = num_match.as_str().parse::<i64>().unwrap();
            if let Some(comment_match) = captures.name("comment") {
              let comment = comment_match.as_str().trim().to_string();
              if let Some(rule_match) = captures.name("rule") {
                let rule = rule_match.as_str().trim().to_string();
                actions.push((num, comment, rule));
              }
            }
          }
        }
      }
    }
  }
  assert!(!actions.is_empty(), "no semantic actions found");
  actions
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn test_extractor_001() {
    extract_numeric_table("", "MY_TABLE");
  }

  #[test]
  #[should_panic]
  fn test_extractor_002() {
    let input = r#"static const yytype_uint16 yystos[] = { 0, 1, 2 };"#;
    extract_numeric_table(input, "yystos");
  }

  #[test]
  #[should_panic]
  fn test_extractor_003() {
    extract_semantic_actions("");
  }

  #[test]
  #[should_panic]
  fn test_extractor_004() {
    extract_value("", "a");
  }

  #[test]
  #[should_panic]
  fn test_extractor_005() {
    extract_symbol_kinds("");
  }

  #[test]
  #[should_panic]
  fn test_extractor_006() {
    extract_token_types("");
  }
}
