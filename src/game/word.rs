use serde::Serialize;

use super::parse;

use std::{
    convert::Infallible,
    fmt,
    io::{BufRead, BufReader, Error, Read},
    str::FromStr,
};

#[derive(Debug, Serialize)]
pub struct Word {
    value: String,
}

impl FromStr for Word {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Word {
            value: String::from_str(s)?,
        })
    }
}

impl parse::Parse for Word {
    //fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error>
    fn parse(r: &mut BufReader<impl Read>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut value = String::new();
        r.read_line(&mut value)?;

        let value = String::from(value.trim())
            .replace("\n", "")
            .replace("\"", "");

        let word = Word { value: value };
        Ok(word)
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
