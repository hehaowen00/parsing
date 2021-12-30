pub mod byte_parser;
pub mod str_parser;

#[cfg(test)]
mod testing;

pub mod error;
pub mod combinator;
pub mod matcher;
pub mod parser;

pub mod str {
    pub use crate::str_parser::ParseStr;
    use crate::combinator::*;
    use crate::matcher::*;

    pub fn many0<'a, P>(p: P) -> Many0<P>
    where
        P: ParseStr<'a>,
    {
        Many0::new(p)
    }

    pub fn many1<'a, P>(p: P) -> Many1<P>
    where
        P: ParseStr<'a>,
    {
        Many1::new(p)
    }

    pub fn one_byte(byte: u8) -> One<u8> {
        One::<u8>::new(byte)
    }

    pub fn one_char(ch: char) -> Seq<u8> {
        let s = String::from(ch);
        str_seq(&s)
    }

    pub fn str_seq(s: &str) -> Seq<u8> {
        Seq::<char>::new(s)
    }

    pub fn take_until<'a, P>(p: P) -> TakeUntil<P>
    where
        P: ParseStr<'a>,
    {
        TakeUntil::new(p)
    }

    pub fn take_while<'a, F>(f: F) -> TakeWhile<F>
    where
        F: Fn(&'a str) -> bool,
    {
        TakeWhile::new(f)
    }
}

pub mod prelude {
    pub use super::error::ParseError;
    pub use super::combinator::{And, Many0, Many1, ManyN, Map, Optional, Or, Skip, State, TakeUntil, TakeWhile};
    pub use super::matcher::{Any, Digit, Letter, One, OneOf, Seq, Whitespace};
    pub use super::util::whitespace;
}

pub mod util {
    use super::matcher::Whitespace;

    pub fn whitespace() -> Whitespace {
        Whitespace::new()
    }
}
