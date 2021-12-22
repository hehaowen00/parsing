use crate::parser::Parse;
use crate::error::ParseError;
use std::marker::PhantomData;

pub struct State<F> {
    init: F,
}

impl<F> State<F> {
    pub fn new(init: F) -> Self {
        Self {
            init,
        }
    }
}

impl<'a, F, T, I> Parse<'a, I> for State<F>
where
    F: Fn() -> T + Clone,
{
    type Output = T;

    #[inline]
    fn parse(&self, input: &'a [I]) -> Result<(Self::Output, &'a [I]), ParseError> {
        let state = (self.init)();
        Ok((state, input))
    }
}

pub struct Map<P, F> {
    p: P,
    f: F,
}

impl<P, F> Map<P, F> {
    pub fn new(p: P, f: F) -> Self {
        Self {
            p,
            f
        }
    }
}

impl<'a, I, P, F, A, B> Parse<'a, I> for Map<P, F>
where
    F: Fn(A) -> B,
    P: Parse<'a, I, Output = A>,
{
    type Output = B;

    fn parse(&self, input: &'a [I]) -> Result<(Self::Output, &'a [I]), ParseError> {
        self.p.parse(input).map(|(res, input)| {
            let b = (self.f)(res);
            (b, input)
        })
    }
}

// zero or more
pub struct Many0<P> {
    p: P
}

impl<P> Many0<P> {
    pub fn new(p: P) -> Self {
        Self {
            p
        }
    }
}

impl<'a, I, P> Parse<'a, I> for Many0<P>
where
    P: Parse<'a, I>,
{
    type Output = Vec<P::Output>;

    fn parse(&self, input: &'a [I]) -> Result<(Self::Output, &'a [I]), ParseError> {
        let mut acc = Vec::new();
        let mut cursor = input;

        while let Ok((res, input)) = self.p.parse(cursor) {
            acc.push(res);
            cursor = input;
        }

        Ok((acc, cursor))
    }
}

// one or more
pub struct Many1<P> {
    p: P
}

impl<P> Many1<P> {
    pub fn new(p: P) -> Self {
        Self {
            p
        }
    }
}

impl<'a, I, P> Parse<'a, I> for Many1<P> 
where
    I: 'a,
    P: Parse<'a, I>,
{
    type Output = Vec<P::Output>;

    fn parse(&self, input: &'a [I]) -> Result<(Self::Output, &'a [I]), ParseError> {
        match input.len() {
            0 => Err(ParseError::Indeterminate),
            _ => {
                let mut acc = Vec::new();
                let mut cursor = input;

                let (res, input) = self.p.parse(cursor)?;
                acc.push(res);
                cursor = input;

                while let Ok((res, input)) = self.p.parse(cursor) {
                    acc.push(res);
                    cursor = input;
                }

                Ok((acc, cursor))
            }
        }
    }
}

// n
pub struct ManyN<P> {
    n: usize,
    p: P,
}

impl<P> ManyN<P> {
    pub fn new(p: P, n: usize) -> Self {
        Self {
            n,
            p,
        }
    }
}

impl<'a, I, P> Parse<'a, I> for ManyN<P>
where
    P: Parse<'a, I>
{
    type Output = Vec<P::Output>;

    fn parse(&self, input: &'a [I]) -> Result<(Self::Output, &'a [I]), ParseError> {
        let mut acc = Vec::with_capacity(self.n);
        let mut cursor = input;

        for _ in 0..self.n {
            let (res, input) = self.p.parse(cursor)?;
            cursor = input;
            acc.push(res);
        }

        Ok((acc, cursor))
    }
}

pub struct Optional<P> {
    p: P
}

impl<P> Optional<P> {
    pub fn new(p: P) -> Self {
        Self {
            p
        }
    }
}

impl<'a, I, P> Parse<'a, I> for Optional<P>
where
    P: Parse<'a, I>
{
    type Output = Option<P::Output>;

    fn parse(&self, input: &'a [I]) -> Result<(Self::Output, &'a [I]), ParseError> {
        match self.p.parse(input) {
            Ok((output, input)) => Ok((Some(output), input)),
            Err(_) => Ok((None, input)),
        }
    }
}

pub struct And<P1, P2> {
    p1: P1,
    p2: P2,
}

impl<P1, P2> And<P1, P2> {
    pub fn new(p1: P1, p2: P2) -> Self {
        Self {
            p1,
            p2
        }
    }
}

impl<'a, I, P1, P2> Parse<'a, I> for And<P1, P2>
where
    P1: Parse<'a, I>,
    P2: Parse<'a, I>,
{
    type Output = (P1::Output, P2::Output);

    fn parse(&self, input: &'a [I]) -> Result<(Self::Output, &'a [I]), ParseError> {
        let (r1, input) = self.p1.parse(input)?;
        let (r2, input) = self.p2.parse(input)?;
        Ok(((r1, r2), input))
    }
}

pub struct Or<P1, P2> {
    p1: P1,
    p2: P2,
}

impl<P1, P2> Or<P1, P2> {
    pub fn new(p1: P1, p2: P2) -> Self {
        Self {
            p1,
            p2,
        }
    }
}

/*
impl<'a, I, A, B, P1, P2> Parse<'a, I> for Or<P1, P2>
where
    P1: Parse<'a, I, Output = A>,
    P2: Parse<'a, I, Output = B>,
{
    type Output = (Option<A>, Option<B>);

    fn parse(&self, input: &'a [I]) -> Result<(Self::Output, &'a [I]), ParseError> {
        match self.p1.parse(input) {
            Ok((r1, input)) => Ok(((Some(r1), None), input)),
            Err(_) => {
                let (r2, input) = self.p2.parse(input)?;
                Ok(((None, Some(r2)), input))
            }
        }
    }
}
*/

impl<'a, I, O, P1, P2> Parse<'a, I> for Or<P1, P2>
where
    P1: Parse<'a, I, Output = O>,
    P2: Parse<'a, I, Output = O>,
{
    type Output = O;

    fn parse(&self, input: &'a [I]) -> Result<(Self::Output, &'a [I]), ParseError> {
        match self.p1.parse(input) {
            Ok((r1, input)) => Ok((r1, input)),
            Err(_) => self.p2.parse(input),
        }
    }
}

pub struct Skip<S: SkipDirection, P1, P2> {
    p1: P1,
    p2: P2,
    marker: PhantomData<S>
}

impl<P1, P2> Skip<Left, P1, P2> {
    pub fn new(p1: P1, p2: P2) -> Self {
        Self {
            p1,
            p2,
            marker: PhantomData,
        }
    }
}

impl<P1, P2> Skip<Right, P1, P2> {
    pub fn new(p1: P1, p2: P2) -> Self {
        Self {
            p1,
            p2,
            marker: PhantomData,
        }
    }
}

impl<'a, I, P1, P2> Parse<'a, I> for Skip<Left, P1, P2>
where
    P1: Parse<'a, I>,
    P2: Parse<'a, I>,
{
    type Output = P2::Output;

    fn parse(&self, input: &'a [I]) -> Result<(Self::Output, &'a [I]), ParseError> {
        let (_, input) = self.p1.parse(input)?;
        self.p2.parse(input)
    }
}

impl<'a, I, P1, P2> Parse<'a, I> for Skip<Right, P1, P2>
where
    P1: Parse<'a, I>,
    P2: Parse<'a, I>,
{
    type Output = P1::Output;

    fn parse(&self, input: &'a [I]) -> Result<(Self::Output, &'a [I]), ParseError> {
        let (output, input) = self.p1.parse(input)?;
        let (_, input) = self.p2.parse(input)?;
        Ok((output, input))
    }
}

pub trait SkipDirection {
}

pub struct Left;

impl SkipDirection for Left {
}

pub struct Right;

impl SkipDirection for Right {
}

pub struct TakeUntil<P> {
    p: P
}

impl<P> TakeUntil<P> {
    pub fn new(p: P) -> Self {
        Self {
            p
        }
    }
}

impl<'a, I, P> Parse<'a, I> for TakeUntil<P>
where
    P: Parse<'a, I>,
    I: 'a,
{
    type Output = &'a [I];

    fn parse(&self, input: &'a [I]) -> Result<(Self::Output, &'a [I]), ParseError> {
        if input.len() == 0 {
            return Err(ParseError::EOF);
        }

        let mut idx = 0;

        while let Err(_) = self.p.parse(&input[idx..]) {
            idx += 1;
            if idx >= input.len() {
                return Ok((&input[..idx], &input[idx..]));
            }
        }

        Ok((&input[..idx], &input[idx..]))
    }
}

