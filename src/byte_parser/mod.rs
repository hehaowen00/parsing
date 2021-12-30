mod combinator;
mod matcher;

pub use combinator::*;
pub use matcher::*;

use crate::combinator::{And, Map, Or};
use crate::combinator::{Skip, Left, Right, SkipN};

pub trait ParseByte<'a> {
    type Output;

    fn parse(&self, input: &'a [u8]) -> Result<(Self::Output, &'a [u8]), &'a [u8]>;

    fn then<P>(self, other: P) -> And<Self, P>
    where
        P: ParseByte<'a>,
        Self: Sized + 'static,
    {
        And::new(self, other)
    }

    fn map<F, B>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Self::Output) -> B,
        Self: Sized,
    {
        Map::new(self, f)
    }

    fn or<P>(self, other: P) -> Or<Self, P>
    where
        P: ParseByte<'a>,
        Self: Sized + 'static,
    {
        Or::new(self, other)
    }

    fn skip_left<P>(self, other: P) -> Skip<Left, Self, P>
    where
        P: ParseByte<'a>,
        Self: Sized + 'static,
    {
        Skip::<Left, _, _>::new(self, other)
    }

    fn skip_right<P>(self, other: P) -> Skip<Right, Self, P>
    where
        P: ParseByte<'a>,
        Self: Sized + 'static,
    {
        Skip::<Right, _, _>::new(self, other)
    }

    fn skip_n(self, n: usize) -> SkipN<Self>
    where
        Self: Sized + 'static,
    {
        SkipN::new(self, n)
    }
}
