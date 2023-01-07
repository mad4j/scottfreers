use super::parse;

use std::{
    convert::Infallible,
    fmt,
    fs::File,
    io::{BufRead, BufReader, Error},
    str::FromStr,
};

#[derive(Debug)]
pub struct Text {
    value: String,
}

impl FromStr for Text {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Text {
            value: String::from_str(s)?,
        })
    }
}

impl parse::Parse for Text {
    fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut value = String::new();

        let mut count = 0;
        while !value.ends_with("\"\n") || (value.starts_with("\"\n") && count == 1) {
            r.read_line(&mut value)?;
            count += 1;
            //TODO: infinite loop in case of bad formatted strings
        }

        let value = String::from(value.trim())
            .replace("\n", "")
            .replace("\"", "");

        let text = Text { value: value };
        Ok(text)
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
