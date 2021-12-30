pub struct Any;

impl Any {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Digit;

impl Digit {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Letter;

impl Letter {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct One<T> {
    pub(crate) len: usize,
    pub(crate) val: T,
}

impl One<u8> {
    pub fn new(byte: u8) -> Self {
        Self {
            len: 0,
            val: byte,
        }
    }
}

pub struct OneOf<T> {
    pub(crate) xs: Vec<T>,
}

impl OneOf<String> {
    pub fn from<'a>(xs: &[&'a str]) -> Self {
        Self {
            xs: xs.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl OneOf<char> {
    pub fn from(s: &str) -> OneOf<[u8; 4]> {
        let mut xs = Vec::new();
        for ch in s.chars() {
            let mut buf = [0u8; 4];
            ch.encode_utf8(&mut buf);
            xs.push(buf);
        }

        OneOf {
            xs
        }
    }
}

pub struct Seq<T> {
    pub(crate) len: usize,
    pub(crate) seq: Vec<T>,
}

impl Seq<char> {
    pub fn new(s: &str) -> Seq<u8> {
        let chars: Vec<_> = s.chars().collect();
        let len = chars.len();
        Seq {
            seq: s.as_bytes().to_vec(),
            len
        }
    }
}

impl Seq<u8> {
    pub fn new(s: &[u8]) -> Self {
        Self {
            seq: s.to_vec(),
            len: s.len(),
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
