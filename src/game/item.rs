use std::{
    fmt,
    io::{BufRead, BufReader, Error, ErrorKind, Read},
};

use log::trace;
use regex::Regex;

use serde::Serialize;

use super::parse;

#[derive(Debug, Serialize)]
pub struct Item {
    text: String,
    auto_get: Option<String>,
    location: u8,
    initial_location: u8,
}

impl parse::Parse for Item {
    fn parse(r: &mut BufReader<impl Read>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut value = String::new();
        while value.matches('"').count() < 2 {
            r.read_line(&mut value)?;
            //TODO: hanging on bad formatted strings
        }

        trace!("parsing raw string {:?} as item...", value);

        let re =
            Regex::new(r#"(?ms)\A"(?P<text>.*?)(?P<auto_get>/(.*?)/)?"\s*(?P<loc>\d{1,3})\s*\z"#)
                .unwrap();

        let cap = re
            .captures(&value)
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
            auto_get,
        };

        Ok(item)
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or(String::from("None"))
        )
    }
}
