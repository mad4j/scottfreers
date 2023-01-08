use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

use regex::Regex;

use super::parse;

#[derive(Debug)]
pub struct Item {
    text: String,
    auto_get: String,
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

        let re = Regex::new(r#"^"(.*?)"\s*(\d{1,3})\s*$"#)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        println!("'{:?}' - {}", line, re.is_match(&line));

        let cap = re.captures(&line).unwrap();
        println!("'{}' - {}", &cap[1], &cap[2]);

        let loc =
            u8::from_str_radix(&cap[2], 10).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        let item = Item {
            text: String::from(&cap[1]),
            location: loc,
            initial_location: loc,
            auto_get: String::new(),
        };

        Ok(item)
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Item {{ text: \"{}\", location: {}, initial_location: {}, auto_get: \"{}\" }}",
            self.text, self.location, self.initial_location, self.auto_get
        )
    }
}
