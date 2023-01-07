use super::parse;

use std::{
    fmt,
    fs::File,
    io::{BufReader, Error},
};

#[derive(Debug)]
pub struct Header {
    pub unknown: u16,
    pub num_items: u16,
    pub num_actions: u16,
    pub num_words: u16, /* Smaller of verb/noun is padded to same size */
    pub num_rooms: u16,
    pub max_carry: u16,
    pub player_room: u16,
    pub num_treasures: u16,
    pub word_length: u16,
    pub light_time: u16,
    pub num_messages: u16,
    pub treasure_room: u16,
}

impl parse::Parse for Header {
    fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let h = Header {
            unknown: Self::parse_u16(r)?,
            num_items: Self::parse_u16(r)?,
            num_actions: Self::parse_u16(r)? + 1,
            num_words: Self::parse_u16(r)? + 1,
            num_rooms: Self::parse_u16(r)? + 1,
            max_carry: Self::parse_u16(r)?,
            player_room: Self::parse_u16(r)?,
            num_treasures: Self::parse_u16(r)?,
            word_length: Self::parse_u16(r)?,
            light_time: Self::parse_u16(r)?,
            num_messages: Self::parse_u16(r)?,
            treasure_room: Self::parse_u16(r)?,
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
