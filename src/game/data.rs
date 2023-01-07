use super::{action::Action, header::Header, parse::Parse, word::Word, room::Room};

use std::{
    fmt,
    fs::File,
    io::{BufReader, Error},
};

use log::{info, debug, trace};

pub struct Data {
    pub header: Header,
    pub actions: Vec<Action>,
    pub verbs: Vec<Word>,
    pub noums: Vec<Word>,
    pub rooms: Vec<Room>,
}

impl Parse for Data {
    fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        info!("Parsing Header");
        let header = Header::parse(r)?;
        debug!("Parsed Header: {:?}", header);

        info!("Parsing Actions");
        let mut actions = Vec::new();
        for _ in 0..header.num_actions {
            actions.push(Action::parse(r)?);
        }
        debug!("Parsed {} actions.", actions.len());

        info!("Parsing Verbs and Noums");
        let mut verbs = Vec::new();
        let mut noums = Vec::new();
        for _ in 0..header.num_words {
            verbs.push(Word::parse(r)?);
            debug!("parsed Verb: {}", verbs.last().unwrap());
            noums.push(Word::parse(r)?);
            debug!("parsed Noum: {}", noums.last().unwrap());
        }

        info!("Parsing Rooms");
        let mut rooms = Vec::new();
        for _ in 0..header.num_rooms {
            trace!("parsing room...");
            rooms.push(Room::parse(r)?);
            debug!("parsed Room: {}", rooms.last().unwrap());
        }

        let data = Data {
            header: header,
            actions: actions,
            verbs: verbs,
            noums: noums,
            rooms: rooms,
        };

        Ok(data)
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.header)
    }
}

impl Data {
    pub fn dump(&self) {
        println!("{}", self.header);
        for i in 0..self.header.num_actions {
            println!("Action[{}]: {}", i, self.actions[i as usize]);
        }

        for i in 0..self.header.num_words {
            println!("Verb[{}]: {}", i, self.verbs[i as usize]);
        }

        for i in 0..self.header.num_words {
            println!("Noum[{}]: {}", i, self.noums[i as usize]);
        }

        for i in 0..self.header.num_rooms {
            println!("Room[{}]: {}", i, self.rooms[i as usize]);
        }
    }
}
