pub trait ParseBytes<'a> {
    type Output;

    fn parse(&self, input: &'a [u8]) -> Result<(&'a [u8], Self::Output), &'a [u8]>;
}