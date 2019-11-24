use failure::_core::fmt::{Display, Error, Formatter};
use std::io;

#[derive(Debug, Fail)]
pub enum ECnfParserError {
    ReadFail(io::Error),
    /// line_num, line
    FailParseKey(u16, String),
    /// line_num, res_prefix
    FailParse(u16, String),
    /// line_num, line
    IllegalRightMidParen(u16, String),
    /// line_num, line, separator
    UnknownSeparator(u16, String, char),
    /// line_num, line, value
    UnknownValue(u16, String, String),
}

#[cfg(test)]
impl PartialEq for ECnfParserError {
    fn eq(&self, other: &Self) -> bool {
        format!("{}", self) == format!("{}", other)
    }
}

#[cfg(test)]
impl Eq for ECnfParserError {}

impl Display for ECnfParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use ECnfParserError::*;
        return match self {
            ReadFail(io_e) => write!(f, "Read Fail: {}", io_e),
            FailParseKey(n, s) => write!(f, "Fail parse key at line: \"{}\" in {}", s, n),
            FailParse(n, prefix) => {
                write!(f, "Illegal End Line for prefix: \"{}\" in {}", prefix, n)
            }
            IllegalRightMidParen(n, l) => {
                write!(f, "Unknown end mid paren on line: \"{}\" in {}", l, n)
            }
            UnknownSeparator(n, l, sep) => write!(
                f,
                "Unknown key-value separator: \"{}\" on line: \"{}\" in {}",
                sep, l, n
            ),
            UnknownValue(n, l, v) => {
                write!(f, "Unknown value: \"{}\" of line: \"{}\" in {}", v, l, n)
            }
        };
    }
}

impl From<io::Error> for ECnfParserError {
    fn from(e: io::Error) -> Self {
        ECnfParserError::ReadFail(e)
    }
}
