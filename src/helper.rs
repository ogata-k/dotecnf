use failure::_core::str::Chars;
use std::fmt::Write;

/// 大文字のアルファベットであることを判定
pub fn is_large_alphabetic(c: &char) -> bool {
    c.is_ascii_alphabetic() && c.is_ascii_uppercase()
}

/// charsを消費して条件を満たす間の文字列を取得
pub fn to_string_while<F: Fn(&char) -> bool>(chars: &mut Chars, predicate: F) -> String {
    let mut s = String::new();
    let mut _chars = chars.clone();
    let mut u: usize = 0;
    loop {
        // scan on chars
        if let Some(c) = _chars.next() {
            if predicate(&c) {
                let _ = s.write_char(c);
                u += 1;
                continue;
            }
            break;
        }
        break;
    }

    // consume u chars
    if u != 0 {
        for _ in 0..u {
            chars.next().unwrap();
        }
    }
    return s;
}

/// 空白を消費
pub fn consume_whitespace(chars: &mut Chars) {
    let _ = to_string_while(chars, |c| c.is_whitespace());
}

pub fn start_end_with(target: &str, start: char, end: char) -> bool {
    // success ""
    // fail "
    target.len() >= 2
        && target.starts_with(|c: char| c == start)
        && target.ends_with(|c: char| c == end)
}
