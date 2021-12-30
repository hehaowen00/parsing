use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ParseError<S> {
    EOF(S),
    Indeterminate(S),
    Invalid(S),
}

impl<S> fmt::Display for ParseError<S>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EOF(stream) => writeln!(f, "unexpected end of stream `{:?}", stream),
            Self::Indeterminate(stream) => writeln!(f, "not enough data to parse `{:?}`", stream),
            Self::Invalid(stream) => writeln!(f, "match failed `{:?}`", stream),
        }
    }
}

impl<S> Error for ParseError<S>
where
    S: fmt::Debug,
{
}