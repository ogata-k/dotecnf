use failure::_core::str::Chars;
use std::fmt::Write;

/// 大文字のアルファベットであることを判定
pub fn is_large_alphabetic(c: &char) -> bool {
    c.is_ascii_alphabetic() && c.to_uppercase().to_string() == c.to_string()
}

/// charsを消費して条件を満たす間の文字列を取得
pub fn to_string_while<F: Fn(&char) -> bool>(chars: &mut Chars, predicate: F) -> String {
    let mut s = String::new();
    for c in chars.by_ref() {
        if predicate(&c) {
            s.write_char(c);
        }
        break;
    }
    return s;
}

/// 空白を消費
pub fn consume_whitespace(chars: &mut Chars) {
    for c in chars.by_ref() {
        if !c.is_whitespace() {
            break;
        }
    }
}

pub fn start_end_with(target: &str, start: char, end: char) -> bool {
    // success ""
    // fail "
    target.len() >= 2
        && target.starts_with(|c: char| c == start)
        && target.ends_with(|c: char| c == end)
}
