use super::parse;

use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub type Token = u16;

impl parse::Parse for Token {
    fn parse(r: &mut BufReader<impl Read>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut l = String::new();
        r.read_line(&mut l)?;

        let v = l
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        Ok(v)
    }
}
