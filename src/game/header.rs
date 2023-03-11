use serde::Serialize;

use super::parse;
use super::token::Token;

use std::{
    fmt,
    io::{BufReader, Error, Read},
};

#[derive(Debug, Serialize)]
pub struct Header {
    pub magic: Token,
    pub num_items: Token,
    pub num_actions: Token,
    pub num_words: Token, /* Smaller of verb/noun is padded to same size */
    pub num_rooms: Token,
    pub max_carry: Token,
    pub player_room: Token,
    pub num_treasures: Token,
    pub word_length: Token,
    pub light_time: Token,
    pub num_messages: Token,
    pub treasure_room: Token,
}

impl parse::Parse for Header {
    fn parse(r: &mut BufReader<impl Read>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let h = Header {
            magic: Token::parse(r)?,
            num_items: Token::parse(r)? + 1,
            num_actions: Token::parse(r)? + 1,
            num_words: Token::parse(r)? + 1,
            num_rooms: Token::parse(r)? + 1,
            max_carry: Token::parse(r)?,
            player_room: Token::parse(r)?,
            num_treasures: Token::parse(r)?,
            word_length: Token::parse(r)?,
            light_time: Token::parse(r)?,
            num_messages: Token::parse(r)? + 1,
            treasure_room: Token::parse(r)?,
        };

        Ok(h)
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or(String::from("None"))
        )
    }
}
