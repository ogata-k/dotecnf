use crate::error::ECnfParserError;
use crate::error::ECnfParserError::{IllegalRightMidParen, UnknownSeparator, UnknownValue};
use crate::helper::{consume_whitespace, is_large_alphabetic, start_end_with, to_string_while};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Lines, Read};

pub const PREFIX_SEPARATOR: &'static str = ".";
pub const PREFIX_KEY_SEPARATOR: &'static str = ".";
const KEY_VALUE_SEPARATOR: char = ':';

/// Parser for Environment-CoNF
pub struct ECnfParser {
    line_num: u16,
    ecnf: HashMap<String, Option<String>>,
    prefix_stack: Vec<String>,
}

impl ECnfParser {
    pub fn new() -> Self {
        ECnfParser {
            line_num: 0,
            ecnf: Default::default(),
            prefix_stack: vec![],
        }
    }

    /// scanデータからECnfを構築
    pub fn build_ecnf(&self) -> HashMap<String, Option<String>> {
        self.ecnf.clone()
    }

    /// prefixを結合する
    fn join_prefix(&self) -> String {
        self.prefix_stack.join(PREFIX_SEPARATOR)
    }

    fn build_key(&self, key: &str) -> String {
        if self.prefix_stack.is_empty() {
            key.to_string()
        } else {
            format!("{}{}{}", self.join_prefix(), PREFIX_KEY_SEPARATOR, key)
        }
    }

    // TODO 後はファイルとかパスとかから直接読み取れるようにヘルパも作っておく
    /// 入力を元にECnfをパースする
    pub fn load<R: Read>(&mut self, reader: R) -> Result<(), ECnfParserError> {
        let mut lines = BufReader::new(reader).lines();
        return self._load(&mut lines);
    }

    fn _load<R: Read>(&mut self, lines: &mut Lines<BufReader<R>>) -> Result<(), ECnfParserError> {
        let line = lines.next();
        if line.is_none() {
            if self.prefix_stack.is_empty() {
                // プレフィックスがない（つまり深さが0）ときで、最後まで読んだとき
                return Ok(());
            } else {
                // 階層があるのに最後まで到達した場合
                return Err(ECnfParserError::FailParse(
                    self.line_num,
                    self.join_prefix(),
                ));
            }
        }
        // update line
        self.line_num += 1;

        let line: String = line.unwrap()?;
        let trim_line: &str = line.trim();
        if trim_line.is_empty() {
            return self._load(lines);
        }
        if trim_line.starts_with(|c| c == '#') {
            {
                // #始まりのコメントは読み飛ばす
                return self._load(lines);
            }
        }
        if trim_line == "}" {
            // }だけからなるときの階層を一つ上がる処理
            if self.prefix_stack.is_empty() {
                return Err(IllegalRightMidParen(self.line_num, trim_line.to_string()));
            }
            self.prefix_stack.pop();
            return self._load(lines);
        }
        // key-valueの形状チェック
        if trim_line.starts_with(|c: char| is_large_alphabetic(&c)) {
            let mut chars = trim_line.chars();
            let key: String = to_string_while(&mut chars, |c| is_large_alphabetic(&c) || *c == '_');
            consume_whitespace(&mut chars);
            let sep = chars.next();
            return match &sep {
                Some(c) if *c != KEY_VALUE_SEPARATOR => Err(UnknownSeparator(
                    self.line_num,
                    trim_line.to_string(),
                    sep.unwrap(),
                )),
                None => Err(UnknownSeparator(
                    self.line_num,
                    trim_line.to_string(),
                    char::default(),
                )),
                Some(_) => {
                    consume_whitespace(&mut chars);
                    let res_str: &str = chars.as_str();
                    if res_str == "" {
                        // valueがないとき
                        self.ecnf.insert(self.build_key(&key), None);
                        return self._load(lines);
                    } else {
                    }
                    if res_str == "{" {
                        // 階層を一つ下がる
                        self.prefix_stack.push(key);
                        return self._load(lines);
                    }
                    if start_end_with(res_str, '"', '"') {
                        // value is "hoge"
                        let s: String = if res_str.len() == 2 {
                            "".to_string()
                        } else {
                            res_str[1..res_str.len() - 1].to_string()
                        };
                        self.ecnf.insert(self.build_key(&key), Some(s));
                        return self._load(lines);
                    }
                    return Err(UnknownValue(
                        self.line_num,
                        trim_line.to_string(),
                        res_str.to_string(),
                    ));
                }
            };
        } else {
            Err(ECnfParserError::FailParseKey(
                self.line_num,
                trim_line.to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    mod success_test {
        use crate::parser::ECnfParser;
        use std::collections::HashMap;

        fn success_test_helper(input: &str, result: &[(&str, Option<&str>)]) {
            let mut parser = ECnfParser::new();
            if let Err(e) = parser.load(input.as_bytes()) {
                eprintln!("Err:\t{}", e);
                eprintln!("input:\t\"{}\"", input);
                assert!(false);
            }
            let actual_ecnf: HashMap<String, Option<String>> = parser.build_ecnf();
            let mut expect_ecnf: HashMap<String, Option<String>> = HashMap::new();
            for (key, value) in result {
                if value.is_none() {
                    expect_ecnf.insert(key.to_string(), None);
                } else {
                    expect_ecnf.insert(key.to_string(), Some(value.unwrap().to_string()));
                }
            }
            assert_eq!(actual_ecnf, expect_ecnf);
        }

        #[test]
        fn success_empty_input() {
            let input: &str = r"";
            let result: Vec<(&str, Option<&str>)> = vec![];
            success_test_helper(input, &result);
        }

        #[test]
        fn success_comment_input() {
            let input: &str = r"# hoge i hoge j";
            let result: Vec<(&str, Option<&str>)> = vec![];
            success_test_helper(input, &result);
        }

        #[test]
        fn success_key_not_empty_value_input() {
            let input: &str = r#"  HO_GE : " FU ga ""#;
            let result: Vec<(&str, Option<&str>)> = vec![("HO_GE", Some(" FU ga "))];
            success_test_helper(input, &result);
        }

        #[test]
        fn success_new_line_input() {
            let input: &str = r#"

                HOGE: "FUGA"

                HOOO: ""

            "#;
            let result: Vec<(&str, Option<&str>)> =
                vec![("HOGE", Some("FUGA")), ("HOOO", Some(""))];
            success_test_helper(input, &result);
        }
        #[test]
        fn success_key_empty_value_input() {
            let input: &str = r#"  HO_GE : """#;
            let result: Vec<(&str, Option<&str>)> = vec![("HO_GE", Some(""))];
            success_test_helper(input, &result);
        }

        #[test]
        fn success_key_none_value_input() {
            let input: &str = r#"  HO_GE : "#;
            let result: Vec<(&str, Option<&str>)> = vec![("HO_GE", None)];
            success_test_helper(input, &result);
        }

        #[test]
        fn success_key_value_empty_hierarchy_input() {
            let input: &str = r#"
        DB : {
        }"#;
            let result: Vec<(&str, Option<&str>)> = vec![];
            success_test_helper(input, &result);
        }

        #[test]
        fn success_key_value_hierarchy_input() {
            let input: &str = r#"
        DB : {
            ACCOUNT_NAME :
            PASSWORD :
            PATH : ""
            DRIVER : "SQLite"
        }"#;
            let result: Vec<(&str, Option<&str>)> = vec![
                ("DB.ACCOUNT_NAME", None),
                ("DB.PASSWORD", None),
                ("DB.PATH", Some("")),
                ("DB.DRIVER", Some("SQLite")),
            ];
            success_test_helper(input, &result);
        }

        #[test]
        fn success_key_values_many_empty_hierarchy_input() {
            let input: &str = r#"
        DB : {
            SQLITE: {
            }
            LOG_FILE: {
                DIRECTORY: "./log"
                PATH: "database.log"
            }
        }"#;
            let result: Vec<(&str, Option<&str>)> = vec![
                ("DB.LOG_FILE.DIRECTORY", Some("./log")),
                ("DB.LOG_FILE.PATH", Some("database.log")),
            ];
            success_test_helper(input, &result);
        }

        #[test]
        fn success_key_values_many_hierarchy_input() {
            let input: &str = r#"
        DB : {
            DRIVER : "SQLite"
            SQLITE: {
                ACCOUNT_NAME : "user"
                PASSWORD : "_"3$#"
                PATH : ""
            }
            LOG_FILE: {
                DIRECTORY: "./log"
                PATH: "database.log"
            }
        }"#;
            let result: Vec<(&str, Option<&str>)> = vec![
                ("DB.DRIVER", Some("SQLite")),
                ("DB.SQLITE.ACCOUNT_NAME", Some("user")),
                ("DB.SQLITE.PASSWORD", Some("_\"3$#")),
                ("DB.SQLITE.PATH", Some("")),
                ("DB.LOG_FILE.DIRECTORY", Some("./log")),
                ("DB.LOG_FILE.PATH", Some("database.log")),
            ];
            success_test_helper(input, &result);
        }

        #[test]
        fn success_key_values_input() {
            let input: &str = r#"
        # app version
        VERSION :  "4.2.23"

        # screen
        SCREEN:{
            # empty setting so empty string
            SC_ZERO: "日本語"
            SC_ONE:
            SC_TWO: "default"

            # value is "hoge hoge " and key is SCREEN.SC_THREE
            SC_THREE: "hoge hoge "
        }"#;
            let result: Vec<(&str, Option<&str>)> = vec![
                ("VERSION", Some("4.2.23")),
                ("SCREEN.SC_ZERO", Some("日本語")),
                ("SCREEN.SC_ONE", None),
                ("SCREEN.SC_TWO", Some("default")),
                ("SCREEN.SC_THREE", Some("hoge hoge ")),
            ];
            success_test_helper(input, &result);
        }
    }

    mod error_test {
        use crate::error::ECnfParserError;
        use crate::parser::{ECnfParser, PREFIX_SEPARATOR};

        fn error_test_helper(input: &str, result: ECnfParserError) {
            let mut parser = ECnfParser::new();
            let parse_result = parser.load(input.as_bytes());
            if parse_result.is_ok() {
                eprintln!("Err: Parse success");
                eprintln!("input:\t\"{}\"", input);
                assert!(false);
            }
            assert_eq!(parse_result.err().unwrap(), result);
        }

        #[test]
        fn error_not_head_comment() {
            let input: &str = r#"HOGE: "huge"  # comment"#;
            let err = ECnfParserError::UnknownValue(
                1,
                input.to_string(),
                "\"huge\"  # comment".to_string(),
            );
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
            let err =
                ECnfParserError::UnknownValue(1, input.to_string(), "'vim style'".to_string());
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
    }
}
