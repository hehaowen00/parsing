use crate::parser::*;

pub struct Cell<P> {
    parser: P,
}

impl<'a, P> Cell<P> {
    #[inline]
    pub fn new(parser: P) -> Self {
        Self {
            parser,
        }
    }

    #[inline]
    pub fn take(self) -> P {
        self.parser
    }

    #[inline]
    pub fn map<B, F>(self, f: F) -> Cell<Map<P, F>>
    where
        F: Fn(P::Output) -> B,
        P: Parse<'a>
    {
        Cell::new(Map::new(self.take(), f))
    }

    #[inline]
    pub fn or<RHS>(self, rhs: Cell<RHS>) -> Cell<Or<P, RHS>>
    where
        RHS: Parse<'a, Output = P::Output>,
        P: Parse<'a>
    {
        Cell::new(Or::new(self.take(), rhs.take()))
    }

    #[inline]
    pub fn then<RHS>(self, rhs: Cell<RHS>) -> Cell<And<P, RHS>>
    where
        RHS: Parse<'a>,
    {
        Cell::new(And::new(self.take(), rhs.take()))
    }

    #[inline]
    pub fn skip<RHS>(self, rhs: Cell<RHS>) -> Cell<Skip<P, RHS>>
    where
        RHS: Parse<'a>,
    {
        Cell::new(Skip::new(self.take(), rhs.take()))
    }

    #[inline]
    pub fn skip_left<RHS>(self, rhs: Cell<RHS>) -> Cell<Skip<RHS, P>>
    where
        RHS: Parse<'a>,
    {
        Cell::new(Skip::new(rhs.take(), self.take()))
    }
}

impl<'a, P> Parse<'a> for Cell<P>
where
    P: Parse<'a>,
{
    type Output = P::Output;

    #[inline]
    fn parse(&self, input: &'a [u8]) -> Result<(&'a [u8], Self::Output), &'a [u8]> {
        self.parser.parse(input)
    }
}
