use super::parse;
use super::token::Token;

use std::{
    fmt,
    fs::File,
    io::{BufReader, Error},
};

#[derive(Debug)]
pub struct Header {
    pub unknown: Token,
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
    fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let h = Header {
            unknown: Token::parse(r)?,
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
        let ni = format!("Num of Items    : {}\n", self.num_items);
        let nt = format!("Num of Treasures: {}\n", self.num_treasures);
        let mc = format!("Max Carry       : {}\n", self.max_carry);

        let nr = format!("Num of Rooms    : {}\n", self.num_rooms);
        let pr = format!("Player Room     : {}\n", self.player_room);
        let tr = format!("Treasure Room   : {}\n", self.treasure_room);

        let na = format!("Num of Actions  : {}\n", self.num_actions);
        let nw = format!("Num of Words    : {}\n", self.num_words);
        let wl = format!("Word Lenght     : {}\n", self.word_length);
        let lt = format!("Ligth Time      : {}\n", self.light_time);

        let nm = format!("Num of Messages : {}\n", self.num_messages);

        let un = format!("Unknown         : {}\n", self.unknown);

        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}{}{}",
            ni, nt, mc, nr, pr, tr, na, nw, wl, lt, nm, un
        )
    }
}
