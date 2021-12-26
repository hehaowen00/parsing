use crate::error::ParseError;
use crate::parser::Parse;

pub struct Any;

impl Any {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> Parse<T> for Any {
    type Output<'o> = &'o T;

    fn parse<'a>(&self, input: &'a [T]) -> Result<(Self::Output<'a>, &'a [T]), ParseError> {
        match input.len() {
            0 => Err(ParseError::EOF),
            _ => Ok((&input[0], &input[1..])),
        }
    }
}

pub struct Digit;

impl Digit {
    pub fn new() -> Self {
        Self {}
    }
}

impl Parse<char> for Digit {
    type Output = char;

    fn parse<'a>(&self, input: &'a [char]) -> Result<(Self::Output<'a>, &'a [char]), ParseError> {
        match input.len() {
            0 => Err(ParseError::Indeterminate),
            _ => match input[0].is_numeric() {
                true => Ok((input[0], &input[1..])),
                false => Err(ParseError::Invalid)
            }
        }
    }
}

pub struct Letter;

impl Letter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Parse< char> for Letter {
    type Output = char;

    fn parse<'a>(&self, input: &'a [char]) -> Result<(Self::Output<'a>, &'a [char]), ParseError> {
        match input.len() {
            0 => Err(ParseError::Indeterminate),
            _ => match input[0].is_alphabetic() {
                true => Ok((input[0], &input[1..])),
                false => Err(ParseError::Invalid)
            }
        }
    }
}

pub struct One<T> {
    val: T
}

impl One<char> {
    pub fn new(ch: char) -> Self {
        Self {
            val: ch,
        }
    }
}

impl One<u8> {
    pub fn new(byte: u8) -> Self {
        Self {
            val: byte,
        }
    }
}

impl<T> Parse<T> for One<T> {
    type Output<'o> = &'o T;

    fn parse<'a>(&self, input: &'a [T]) -> Result<(Self::Output<'a>, &'a [T]), ParseError> {
        match input.len() {
            0 => Err(ParseError::EOF),
            _ => match input[0] == self.val {
                true => Ok((&input[0], &input[1..])),
                false => Err(ParseError::Invalid),
            }
        }
    }
}

pub struct OneOf<T> {
    xs: Vec<T>,
}

impl OneOf<char> {
    pub fn from(s: &str) -> Self {
        Self {
            xs: s.chars().collect()
        }
    }
}

impl Parse<char> for OneOf<char> {
    type Output = char;

    fn parse<'a>(&self, input: &'a [char]) -> Result<(Self::Output<'a>, &'a [char]), ParseError> {
        match self.xs.contains(&input[0]) {
            true => Ok((input[0], &input[1..])),
            false => Err(ParseError::Invalid)
        }
    }
}
pub struct Seq<T> {
    seq: Vec<T>,
}

impl Seq<char> {
    pub fn new(s: &str) -> Self {
        Self {
            seq: s.chars().collect(),
        }
    }
}

impl Seq<u8> {
    pub fn new(s: &[u8]) -> Self {
        Self {
            seq: s.to_vec()
        }
    }
}

impl<T> Parse<T> for Seq<T> {
    type Output<'o> = &'o [T];

    fn parse<'a>(&self, input: &'a [T]) -> Result<(Self::Output<'a>, &'a [T]), ParseError> {
        let len = self.seq.len();
        match input.len() >= len {
            true => {
                for i in 0..len {
                    if self.seq[i] != input[i] {
                        return Err(ParseError::Invalid);
                    }
                }

                Ok((&input[0..len], &input[len..]))
            },
            false => {
                Err(ParseError::Indeterminate)
            }
        }
    }
}

pub struct Whitespace;

impl Whitespace {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Parse<char> for Whitespace {
    type Output = char;

    fn parse<'a>(&self, input: &'a [char]) -> Result<(Self::Output<'a>, &'a [char]), ParseError> {
        match input.len() {
            0 => {
                return Err(ParseError::EOF);
            }
            _ => {
                if input[0].is_whitespace() {
                    return Ok((input[0], &input[1..]));
                }
            }
        }

        Err(ParseError::Invalid)
    }
}
