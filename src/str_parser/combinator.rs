use crate::matcher::{Any, Digit, Seq};

use crate::combinator::{And, Many0, Many1, Map, Or, Skip, SkipN, State, Left, Right, TakeUntil};
use crate::str_parser::ParseStr;

impl<'a, F, T> ParseStr<'a> for State<F>
where
    F: Fn() -> T + Clone,
{
    type Output = T;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        let state = (self.init)();
        Ok((state, input))
    }
}

impl<'a, P, F, A, B> ParseStr<'a> for Map<P, F>
where
    F: Fn(A) -> B,
    P: ParseStr<'a, Output = A>,
{
    type Output = B;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        let (a, input) = self.p.parse(input)?;
        let b = (self.f)(a);
        Ok((b, input))
    }
}

impl<'a, P> ParseStr<'a> for Many0<P>
where
    P: ParseStr<'a>
{
    type Output = Vec<P::Output>;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        let mut acc = Vec::new();
        let mut cursor = input;

        while let Ok((res, input)) = self.p.parse(cursor) {
            acc.push(res);
            cursor = input;
        }

        Ok((acc, input))
    }
}

impl<'a, P> ParseStr<'a> for Many1<P>
where
    P: ParseStr<'a>,
{
    type Output = Vec<P::Output>;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        let mut acc = Vec::new();

        let (res, mut cursor) = self.p.parse(input)?;
        acc.push(res);

        while let Ok((res, input)) = self.p.parse(cursor) {
            acc.push(res);
            cursor = input;
        }

        Ok((acc, cursor))
    }
}

impl<'a, P1, P2> ParseStr<'a> for And<P1, P2>
where
    P1: ParseStr<'a>,
    P2: ParseStr<'a>,
{
    type Output = (P1::Output, P2::Output);

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        match self.p1.parse(input) {
            Ok((a, input)) => match self.p2.parse(input) {
                Ok((b, input)) => Ok(((a, b), input)),
                Err(input) => Err(input),
            },
            Err(input) => Err(input),
        }
    }
}

impl<'a, P1, P2, O> ParseStr<'a> for Or<P1, P2>
where
    P1: ParseStr<'a, Output = O>,
    P2: ParseStr<'a, Output = O>,
{
    type Output = O;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        match self.p1.parse(input) {
            Ok(res) => Ok(res),
            Err(input) => self.p2.parse(input),
        }
    }
}

impl<'a, P1, P2> ParseStr<'a> for Skip<Left, P1, P2>
where
    P1: ParseStr<'a>,
    P2: ParseStr<'a>,
{
    type Output = P2::Output;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        let (_, input) = self.p1.parse(input)?;
        self.p2.parse(input)
    }
}

impl<'a, P1, P2> ParseStr<'a> for Skip<Right, P1, P2>
where
    P1: ParseStr<'a>,
    P2: ParseStr<'a>,
{
    type Output = P1::Output;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        let (a, input) = self.p1.parse(input)?;
        let (_, input) = self.p2.parse(input)?;
        Ok((a, input))
    }
}

impl<'a> ParseStr<'a> for TakeUntil<Seq<u8>> {
    type Output = &'a str;

    #[inline]
    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        let s = unsafe { std::str::from_utf8_unchecked(&self.p.seq) };
        match input.find(s) {
            Some(idx) => Ok((&input[..idx], &input[idx..])),
            None => Err(input),
        }
    }
}

impl<'a> ParseStr<'a> for TakeUntil<Any> {
    type Output = &'a str;

    #[inline]
    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        Ok((&input[..], &input[input.len()..]))
    }
}

/*
impl<'a, P> ParseStr<'a> for TakeUntil<P>
where
    P: ParseStr<'a>,
{
    type Output = &'a str;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        let mut cursor = input;
        let mut idx = 0;

        while let Err(_) = self.p.parse(cursor)  {
            idx += 1;

            if idx >= input.len() {
                break;
            }

            cursor = &input[idx..];
        }

        if idx > input.len() {
            return Err(input);
        }

        Ok((&input[0..idx], &input[idx..]))
    }
}
*/

impl<'a, P> ParseStr<'a> for SkipN<P>
where
    P: ParseStr<'a>
{
    type Output = P::Output;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        let (res, xs) = self.p.parse(input)?;
        Ok((res, &xs[self.n..]))
    }
}
