use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind}, fmt,
};

#[derive(Debug)]
pub struct GameHeader {
    unknown: u16,
    num_items: u16,
    num_actions: u16,
    num_words: u16, /* Smaller of verb/noun is padded to same size */
    num_rooms: u16,
    max_carry: u16,
    player_room: u16,
    num_treasures: u16,
    word_length: u16,
    light_time: u16,
    num_messages: u16,
    treasure_room: u16,
}

impl GameHeader {
    fn new(
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
    ) -> GameHeader {
        GameHeader {
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

    pub fn parse(f: &mut File) -> Result<GameHeader, Error> {
        let mut r = BufReader::new(f);

        let un = Self::parse_u16(&mut r)?;
        let ni = Self::parse_u16(&mut r)?;
        let na = Self::parse_u16(&mut r)?;
        let nw = Self::parse_u16(&mut r)?;
        let nr = Self::parse_u16(&mut r)?;
        let mc = Self::parse_u16(&mut r)?;
        let pr = Self::parse_u16(&mut r)?;
        let nt = Self::parse_u16(&mut r)?;
        let wl = Self::parse_u16(&mut r)?;
        let lt = Self::parse_u16(&mut r)?;
        let nm = Self::parse_u16(&mut r)?;
        let tr = Self::parse_u16(&mut r)?;

        let gm = GameHeader::new(un, ni, na, nw, nr, mc, pr, nt, wl, lt, nm, tr);
        Ok(gm)
    }

    fn parse_u16(r: &mut BufReader<&mut File>) -> Result<u16, Error> {
        let mut l = String::new();
        r.read_line(&mut l)?;

        let v = l
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        Ok(v)
    }
}

impl fmt::Display for GameHeader {
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

        write!(f, "{}{}{}{}{}{}{}{}{}{}{}{}", ni, nt, mc, nr, pr, tr, na, nw, wl, lt, nm, un)
    }
}