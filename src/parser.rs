use crate::combinator::{Map, And, Or, Skip, Left, Right};
use crate::error::ParseError;

pub trait Parse<'a, I> {
    type Output;

    fn parse(&self, input: &'a [I]) -> Result<(Self::Output, &'a [I]), ParseError>;

    fn then<P>(self, other: P) -> And<Self, P>
    where
        P: Parse<'a, I>,
        Self: Sized,
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
        P: Parse<'a, I>,
        Self: Sized,
    {
        Or::new(self, other)
    }

    fn skip_left<P>(self, other: P) -> Skip<Left, Self, P>
    where
        P: Parse<'a, I>,
        Self: Sized,
    {
        Skip::<Left, _, _>::new(self, other)
    }

    fn skip_right<P>(self, other: P) -> Skip<Right, Self, P>
    where
        P: Parse<'a, I>,
        Self: Sized,
    {
        Skip::<Right, _, _>::new(self, other)
    }
}
