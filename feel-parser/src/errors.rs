//! Implementation of errors for FEEL parser.

use dmntk_common::{DmntkError, ToErrorMessage};

/// Lexer error.
#[derive(ToErrorMessage)]
struct LexerError(String);

pub fn err_unexpected_eof() -> DmntkError {
  LexerError("unexpected end of file".to_string()).into()
}

pub fn err_expected_hex_digit(ch: char) -> DmntkError {
  LexerError(format!("expected hex digit but encountered '{ch}'")).into()
}

pub fn err_unicode_value_out_of_range(value: u64) -> DmntkError {
  LexerError(format!("value is out of allowed Unicode range 0x0000..0x10FFFF : {value:X}")).into()
}

pub fn err_unicode_surrogate_out_of_range(value: u64) -> DmntkError {
  LexerError(format!("surrogate value is out of allowed range 0xD800..0xDFFF : {value:X}")).into()
}

/// Parser error.
#[derive(ToErrorMessage)]
struct ParserError(String);

/// Creates an error when `FEEL` name was expected on input, but something else encountered.
pub fn err_not_a_feel_name(s: &str) -> DmntkError {
  ParserError(format!("expected `FEEL` name on input but found `{s}`")).into()
}

/// Creates syntax error on specified input.
pub fn err_syntax_error(input: &str) -> DmntkError {
  ParserError(format!("syntax error: {input}")).into()
}
