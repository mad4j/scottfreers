use serde::Serialize;

use super::{parse, text::Text, token::Token};

use std::{
    fmt,
    io::{BufReader, Error, Read},
};

#[derive(Debug, Serialize)]
pub struct Room {
    pub text: Text,
    pub exits: [Token; 6],
}

impl parse::Parse for Room {
    fn parse(r: &mut BufReader<impl Read>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let room = Room {
            exits: [
                Token::parse(r)?,
                Token::parse(r)?,
                Token::parse(r)?,
                Token::parse(r)?,
                Token::parse(r)?,
                Token::parse(r)?,
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
            "Room {{ text: {}, exits: [{}, {}, {}, {}, {}, {}]}}",
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
