#![feature(generic_associated_types)]
pub mod error;
pub mod combinator;
pub mod matcher;
pub mod parser;
pub mod testing;

pub mod prelude {
    pub use super::parser::Parse;
    pub use super::error::ParseError;
    pub use super::combinator::{State, Map, And, Optional, Or, Many0, Many1, ManyN, Skip, TakeUntil};
    pub use super::matcher::{Any, Digit, Letter, One, OneOf, Seq, Whitespace};
    pub use super::util::{many0, many1, state, take_until, digit, letter, one_byte, one_char, byte_seq, str_seq, whitespace};
}

pub mod util {
    use crate::parser::Parse;
    use crate::combinator::*;
    use crate::matcher::*;

    pub fn many0<I, P>(p: P) -> Many0<P>
    where
        P: Parse<I>
    {
        Many0::new(p)
    }

    pub fn many1<I, P>(p: P) -> Many1<P>
    where
        P: Parse<I>
    {
        Many1::new(p)
    }

    pub fn state<F, T>(f: F) -> State<F>
    where
        F: Fn() -> T,
    {
        State::new(f)
    }
    
    pub fn take_until<I, P>(p: P) -> TakeUntil<P>
    where
        P: Parse<I>
    {
        TakeUntil::new(p)
    }
    
    pub fn digit() -> Digit {
        Digit::new()
    }

    pub fn letter() -> Letter {
        Letter::new()
    }

    pub fn one_byte(b: u8) -> One<u8> {
        One::<u8>::new(b)
    }

    pub fn one_char(ch: char) -> One<char> {
        One::<char>::new(ch)
    }

    pub fn byte_seq(s: &[u8]) -> Seq<u8> {
        Seq::<u8>::new(s)
    }

    pub fn str_seq(s: &str) -> Seq<char> {
        Seq::<char>::new(s)
    }

    pub fn whitespace() -> Whitespace {
        Whitespace::new()
    }
}
