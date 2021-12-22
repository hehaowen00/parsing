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
    
    pub fn many0<'a, I, P>(p: P) -> Many0<P>
    where
        P: Parse<'a, I>
    {
        Many0::new(p)
    }

    pub fn many1<'a, I, P>(p: P) -> Many1<P>
    where
        P: Parse<'a, I>
    {
        Many1::new(p)
    }

    pub fn pbyte(b: u8) -> One<u8> {
        One::<u8>::new(b)
    }

    pub fn pchar(ch: char) -> One<char> {
        One::<char>::new(ch)
    }

    pub fn state<F, T>(f: F) -> State<F>
    where
        F: Fn() -> T,
    {
        State::new(f)
    }
    
    pub fn take_until<'a, I, P>(p: P) -> TakeUntil<P>
    where
        P: Parse<'a, I>
    {
        TakeUntil::new(p)
    }

    pub fn whitespace() -> Whitespace {
        Whitespace::new()
    }
}
