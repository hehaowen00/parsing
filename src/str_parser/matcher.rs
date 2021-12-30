use std::cmp::Ordering;

use crate::str_parser::ParseStr;
use crate::matcher::{Any, Digit, Letter, OneOf, Seq, Whitespace};

impl<'a> ParseStr<'a> for Any {
    type Output = &'a str;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        match input.len() {
            0 => Err(input), _ => Ok((&input[0..1], &input[1..]))
        }
    }
}

impl<'a> ParseStr<'a> for Digit {
    type Output = &'a str;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        match input.chars().next() {
            Some(ch) if ch.is_numeric() => {
                Ok((&input[0..1], &input[1..]))
            },
            _ => Err(input),
        }
    }
}

impl<'a> ParseStr<'a> for Letter {
    type Output = &'a str;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        match input.chars().next() {
            Some(ch) if ch.is_numeric() => {
                Ok((&input[0..1], &input[1..]))
            },
            _ => Err(input),
        }
    }
}

impl<'a> ParseStr<'a> for OneOf<String> {
    type Output = &'a str;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        for s in &self.xs {
            if input.starts_with(s.as_str()) {
                return Ok((&input[..s.len()], &input[s.len()..]));
            }
        }

        Err(input)
    }
}

impl<'a> ParseStr<'a> for OneOf<[u8; 4]> {
    type Output = &'a str;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        let bytes = input.as_bytes();

        let mut temp = [0u8; 4];
        temp.copy_from_slice(&bytes[0..4]);

        match self.xs.contains(&temp) {
            true => Ok((&input[0..1], &input[1..])),
            _ => Err(input),
        }
    }
}

impl<'a> ParseStr<'a> for Seq<u8> {
    type Output = &'a str;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        let bytes = input.as_bytes();
        let len = self.len;
        let seq = &self.seq;

        if bytes.len() < len {
            return Err(input);
        }

        match &bytes[0..len] == seq {
            true => Ok((&input[0..len], &input[len..])),
            false => Err(input)
        }
    }
}

impl<'a> ParseStr<'a> for Whitespace {
    type Output = &'a str;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), &'a str> {
        match input.chars().next() {
            Some(ch) if ch.is_whitespace() => {
                Ok((&input[0..1], &input[1..]))
            },
            _ => Err(input),
        }
    }
}

