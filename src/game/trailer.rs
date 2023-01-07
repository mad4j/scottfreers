use super::{parse, token::Token};

use std::{
    fmt,
    fs::File,
    io::{BufReader, Error},
};
#[derive(Debug)]
pub struct Trailer {
    version: Token,
    number: Token,
    unknown: Token,
}

impl parse::Parse for Trailer {
    fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let trailer = Trailer {
            version: Token::parse(r)?,
            number: Token::parse(r)?,
            unknown: Token::parse(r)?,
        };

        Ok(trailer)
    }
}

impl fmt::Display for Trailer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ version: {}.{}, numner: {}, unknown: {} }}",
            self.version / 100,
            self.version % 100,
            self.number,
            self.unknown
        )
    }
}
