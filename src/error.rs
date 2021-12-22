use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ParseError {
    EOF,
    Indeterminate,
    Invalid,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EOF => writeln!(f, "unexpected end of stream"),
            Self::Indeterminate => writeln!(f, "not enough data to parse"),
            Self::Invalid => writeln!(f, "match failed"),
        }
    }
}

impl Error for ParseError {
}
