use crate::combinator::{Map, And, Or, Skip, Left, Right};
use crate::error::ParseError;

pub trait Parse<I> {
    type Output<'o>;

    fn parse<'a>(&self, input: &'a [I]) -> Result<(Self::Output<'a>, &'a [I]), ParseError>;

    fn then<P>(self, other: P) -> And<Self, P>
    where
        P: Parse<I>,
        Self: Sized + 'static,
    {
        And::new(self, other)
    }

    fn map<'a, F, B>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Self::Output<'a>) -> B,
        Self: Sized + 'static,
    {
        Map::new(self, f)
    }

    fn or<P>(self, other: P) -> Or<Self, P>
    where
        P: Parse<I>,
        Self: Sized + 'static,
    {
        Or::new(self, other)
    }

    fn skip_left<P>(self, other: P) -> Skip<Left, Self, P>
    where
        P: Parse<I>,
        Self: Sized + 'static,
    {
        Skip::<Left, _, _>::new(self, other)
    }

    fn skip_right<P>(self, other: P) -> Skip<Right, Self, P>
    where
        P: Parse<I>,
        Self: Sized + 'static,
    {
        Skip::<Right, _, _>::new(self, other)
    }
}
