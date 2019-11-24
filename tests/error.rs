use dotecnf::error::ECnfParserError;
use dotecnf::parser::{ECnfParser, PREFIX_SEPARATOR};
use std::collections::HashMap;

fn eq(this: &ECnfParserError, other: &ECnfParserError) -> bool {
    format!("{}", this) == format!("{}", other)
}

fn error_test_helper(input: &str, result: ECnfParserError) {
    let mut parser = ECnfParser::new();
    let parse_result = parser.load(input.as_bytes());
    if parse_result.is_ok() {
        eprintln!("Err: Parse success");
        eprintln!("input:\t\"{}\"", input);
        assert!(false);
    }
    assert!(eq(&parse_result.err().unwrap(), &result));
}

#[test]
fn error_not_head_comment() {
    let input: &str = r#"HOGE: "huge"  # comment"#;
    let err =
        ECnfParserError::UnknownValue(1, input.to_string(), "\"huge\"  # comment".to_string());
    error_test_helper(input, err);
}

#[test]
fn error_not_usable_under_score_for_key_head_char() {
    let input: &str = r#"_HOGE:"vim style""#;
    let err = ECnfParserError::FailParseKey(1, input.to_string());
    error_test_helper(input, err);
}

#[test]
fn error_not_usable_character_for_key_head_char() {
    let input: &str = r#"v_HOGE:"vim style""#;
    let err = ECnfParserError::FailParseKey(1, input.to_string());
    error_test_helper(input, err);
}

#[test]
fn error_not_usable_character_for_key() {
    let input: &str = r#"HO_gE:"vim style""#;
    let err = ECnfParserError::UnknownSeparator(1, input.to_string(), 'g');
    error_test_helper(input, err);
}

#[test]
fn error_unknown_separator() {
    let input: &str = r#"HOGE="vim style""#;
    let err = ECnfParserError::UnknownSeparator(1, input.to_string(), '=');
    error_test_helper(input, err);
}

#[test]
fn error_unknown_wrapping_quote() {
    let input: &str = r#"HOGE:'vim style'"#;
    let err = ECnfParserError::UnknownValue(1, input.to_string(), "'vim style'".to_string());
    error_test_helper(input, err);
}

#[test]
fn error_empty_hierarchy() {
    let input: &str = r#"}"#;
    let err = ECnfParserError::IllegalRightMidParen(1, input.to_string());
    error_test_helper(input, err);
}

#[test]
fn error_position_right_mid_paren() {
    let input: &str = r#"HOGE:{}"#;
    let err = ECnfParserError::UnknownValue(1, input.to_string(), "{}".to_string());
    error_test_helper(input, err);
}

#[test]
fn error_end_hierarchy() {
    let input: &str = r#"
            DB: {
                CD: {
                }
            "#;
    let err = ECnfParserError::FailParse(5, "DB".to_string());
    error_test_helper(input, err);
}
