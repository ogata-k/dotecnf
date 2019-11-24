use crate::error::ECnfParserError;
use crate::error::ECnfParserError::{IllegalRightMidParen, UnknownSeparator, UnknownValue};
use crate::helper::{consume_whitespace, is_large_alphabetic, start_end_with, to_string_while};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};
use std::path::Path;

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

    pub fn load_from_str(&mut self, input: &str) -> Result<(), ECnfParserError> {
        self.load(input.as_bytes())
    }

    pub fn load_from_file(&mut self, path: &Path) -> Result<(), ECnfParserError> {
        self.load(File::open(path)?)
    }
}
