use crate::combinator::{
    And,
    Many0,
    Many1,
    Map,
    Or,
    Skip,
    SkipN,
    State,
    Left,
    Right,
    TakeUntil,
    TakeWhile
};
use crate::matcher::{Any, Digit, Seq};
use crate::byte_parser::ParseByte;

impl<'a, P1, P2> ParseByte<'a> for And<P1, P2>
where
    P1: ParseByte<'a>,
    P2: ParseByte<'a>,
{
    type Output = (P1::Output, P2::Output);

    fn parse(&self, input: &'a [u8]) -> Result<(Self::Output, &'a [u8]), &'a [u8]> {
        let (a, input) = self.p1.parse(input)?;
        let (b, input) = self.p2.parse(input)?;
        Ok(((a, b), input))
    }
}

impl<'a, P> ParseByte<'a> for Many0<P>
where
    P: ParseByte<'a>,
{
    type Output = Vec<P::Output>;

    fn parse(&self, input: &'a [u8]) -> Result<(Self::Output, &'a [u8]), &'a [u8]> {
        Err(input)
    }
}
