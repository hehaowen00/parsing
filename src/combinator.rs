use crate::parser::Parse;
use crate::error::ParseError;
use std::marker::PhantomData;

pub struct State<F> {
    pub(crate) init: F,
}

impl<F> State<F> {
    pub fn new(init: F) -> Self {
        Self {
            init,
        }
    }
}

pub struct Map<P, F> {
    pub(crate) p: P,
    pub(crate) f: F,
}

impl<P, F> Map<P, F> {
    pub fn new(p: P, f: F) -> Self {
        Self {
            p,
            f
        }
    }
}

// zero or more
pub struct Many0<P> {
    pub(crate) p: P
}

impl<P> Many0<P> {
    pub fn new(p: P) -> Self {
        Self {
            p
        }
    }
}

// one or more
pub struct Many1<P> {
    pub(crate) p: P
}

impl<P> Many1<P> {
    pub fn new(p: P) -> Self {
        Self {
            p
        }
    }
}


// n
pub struct ManyN<P> {
    pub(crate) n: usize,
    pub(crate) p: P,
}

impl<P> ManyN<P> {
    pub fn new(p: P, n: usize) -> Self {
        Self {
            n,
            p,
        }
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

pub struct And<P1, P2> {
    pub(crate) p1: P1,
    pub(crate) p2: P2,
}

impl<P1, P2> And<P1, P2> {
    pub fn new(p1: P1, p2: P2) -> Self {
        Self {
            p1,
            p2
        }
    }
}

pub struct Or<P1, P2> {
    pub(crate) p1: P1,
    pub(crate) p2: P2,
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
impl<'o, I, O, P1, P2> Parse<I> for Or<P1, P2>
where
    P1: Parse<I, Output<'o> = O>,
    P2: Parse<I, Output<'o> = O>,
{
    type Output = O;

    fn parse<'a>(&self, input: &'a [I]) -> Result<(Self::Output<'a>, &'a [I]), ParseError> {
        match self.p1.parse(input) {
            Ok((r1, input)) => Ok((r1, input)),
            Err(_) => self.p2.parse(input),
        }
    }
}
*/

pub struct Skip<S: SkipDirection, P1, P2> {
    pub(crate) p1: P1,
    pub(crate) p2: P2,
    _phantom: PhantomData<S>
}

impl<P1, P2> Skip<Left, P1, P2> {
    pub fn new(p1: P1, p2: P2) -> Self {
        Self {
            p1,
            p2,
            _phantom: PhantomData,
        }
    }
}

impl<P1, P2> Skip<Right, P1, P2> {
    pub fn new(p1: P1, p2: P2) -> Self {
        Self {
            p1,
            p2,
            _phantom: PhantomData,
        }
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

pub struct SkipN<P> {
    pub(crate) p: P,
    pub(crate) n: usize,
}

impl<P> SkipN<P> {
    pub fn new(p: P, n: usize) -> Self {
        Self {
            p,
            n
        }
    }
}

pub struct TakeUntil<P> {
    pub(crate) p: P
}

impl<P> TakeUntil<P> {
    pub fn new(p: P) -> Self {
        Self {
            p
        }
    }
}

pub struct TakeWhile<F> {
    pub(crate) f: F,
}

impl<F> TakeWhile<F> {
    pub fn new(f: F) -> Self {
        Self {
            f
        }
    }
}
