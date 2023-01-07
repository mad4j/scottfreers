use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader, Error},
};

use super::{parse};

#[derive(Debug)]
pub struct Item {
    text: String,
    location: u8,
    initial_location: u8,
    auto_get: String,
}

impl parse::Parse for Item {
    fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut line = String::new();

        r.read_line(&mut line)?;

        let item = Item {
            text: line,
            location: 0,
            initial_location: 0,
            auto_get: String::new(),
        };

        Ok(item)
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Item {{ text: \"{:?}\", location: {}, initial_location: {}, auto_get: \"{:?}\" }}",
            self.text, self.location, self.initial_location, self.auto_get
        )
    }
}
