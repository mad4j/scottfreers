use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

use log::trace;
use regex::Regex;

use super::parse;

#[derive(Debug)]
pub struct Item {
    text: String,
    auto_get: Option<String>,
    location: u8,
    initial_location: u8,
}

impl parse::Parse for Item {
    fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut line = String::new();

        r.read_line(&mut line)?;
        trace!("parsing raw string {:?} as item...", line);

        let re =
            Regex::new(r#"^"(?P<text>.*?)(?P<auto_get>/(.*?)/)?"\s*(?P<loc>\d{1,3})\s*$"#).unwrap();

        let cap = re
            .captures(&line)
            .ok_or(Error::new(ErrorKind::InvalidData, "Not valid Item"))?;

        let text = cap
            .name("text")
            .ok_or(Error::new(
                ErrorKind::InvalidData,
                "Not valid Item 'text' field",
            ))?
            .as_str();

        let auto_get = cap.name("auto_get").map(|s| String::from(s.as_str()));

        let loc = u8::from_str_radix(
            cap.name("loc")
                .ok_or(Error::new(
                    ErrorKind::InvalidData,
                    "Not valid Item 'location' field",
                ))?
                .as_str(),
            10,
        )
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        let item = Item {
            text: String::from(text),
            location: loc,
            initial_location: loc,
            auto_get: auto_get,
        };

        Ok(item)
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Item {{ text: \"{}\", location: {}, initial_location: {}, auto_get: \"{:?}\" }}",
            self.text, self.location, self.initial_location, self.auto_get
        )
    }
}
