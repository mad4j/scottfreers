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

impl Header {
    pub fn new() -> Header {
        Header {
            unknown: 0,
            num_items: 0,
            num_actions: 0,
            num_words: 0,
            num_rooms: 0,
            max_carry: 0,
            player_room: 0,
            num_treasures: 0,
            word_length: 0,
            light_time: 0,
            num_messages: 0,
            treasure_room: 0,
        }
    }

    fn init(
        unknown: u16,
        num_items: u16,
        num_actions: u16,
        num_words: u16,
        num_rooms: u16,
        max_carry: u16,
        player_room: u16,
        num_treasures: u16,
        word_length: u16,
        light_time: u16,
        num_messages: u16,
        treasure_room: u16,
    ) -> Header {
        Header {
            unknown,
            num_items,
            num_actions,
            num_words,
            num_rooms,
            max_carry,
            player_room,
            num_treasures,
            word_length,
            light_time,
            num_messages,
            treasure_room,
        }
    }
}

impl parse::Parse for Header {
    fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error> {

        let h = Header {
            unknown: Self::parse_u16(r)?,
            num_items: Self::parse_u16(r)?,
            num_actions: Self::parse_u16(r)?,
            num_words: Self::parse_u16(r)?,
            num_rooms: Self::parse_u16(r)?,
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
