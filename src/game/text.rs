use serde::Serialize;

use super::parse;

use std::{
    convert::Infallible,
    fmt,
    io::{BufRead, BufReader, Error, Read},
    str::FromStr,
};

#[derive(Debug, Serialize)]
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
    fn parse(r: &mut BufReader<impl Read>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut value = String::new();
        while value.matches(r#"""#).count() < 2 {
            r.read_line(&mut value)?;
            //TODO: hanging on bad formatted strings
        }

        //let mut count = 0;
        //while !value.ends_with("\"\n") || (value.starts_with("\"\n") && count == 1) {
        //    r.read_line(&mut value)?;
        //    count += 1;
        //    //TODO: infinite loop in case of bad formatted strings
        //}

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
