use super::{parse, text::Text};

use std::{
    fmt,
    fs::File,
    io::{BufReader, Error},
};

#[derive(Debug)]
pub struct Room {
    text: Text,
    exits: [u16; 6],
}

impl parse::Parse for Room {
    fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let room = Room {
            exits: [
                Self::parse_u16(r)?,
                Self::parse_u16(r)?,
                Self::parse_u16(r)?,
                Self::parse_u16(r)?,
                Self::parse_u16(r)?,
                Self::parse_u16(r)?,
            ],
            text: Text::parse(r)?,
        };
        Ok(room)
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Room {{ text: {:?}, exits: [{}, {}, {}, {}, {}, {}]}}",
            self.text,
            self.exits[0],
            self.exits[1],
            self.exits[2],
            self.exits[3],
            self.exits[4],
            self.exits[5],
        )
    }
}
