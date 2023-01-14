/*
/////////////////////////////////////////////////////////////////////////

Header Format
Objects,Actions,Words,Rooms,Maxhold,Start,Treasures,Wordlen,Time,Msgs,Troom

Datafile Format:

Header,{Verb0},{Noun0},{Verbn},{Nounn},Actions,Rooms,Messages,Objects
*/

use super::{
    action::Action, header::Header, item::Item, parse::Parse, room::Room, text::Text,
    trailer::Trailer, word::Word,
};

use serde::Serialize;

use std::{
    fmt,
    io::{BufReader, Error, Read},
};

#[derive(Debug, Serialize)]
pub struct Data {
    pub header: Header,
    pub actions: Vec<Action>,
    pub verbs: Vec<Word>,
    pub noums: Vec<Word>,
    pub rooms: Vec<Room>,
    pub messages: Vec<Text>,
    pub items: Vec<Item>,
    pub trailer: Trailer,
}

impl Parse for Data {
    fn parse(r: &mut BufReader<impl Read>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let header = Header::parse(r)?;

        let mut actions = Vec::new();
        for _ in 0..header.num_actions {
            actions.push(Action::parse(r)?);
        }

        let mut verbs = Vec::new();
        let mut noums = Vec::new();
        for _ in 0..header.num_words {
            verbs.push(Word::parse(r)?);
            noums.push(Word::parse(r)?);
        }

        let mut rooms = Vec::new();
        for _ in 0..header.num_rooms {
            rooms.push(Room::parse(r)?);
        }

        let mut messages = Vec::new();
        for _ in 0..header.num_messages {
            messages.push(Text::parse(r)?);
        }

        let mut items = Vec::new();
        for _ in 0..header.num_items {
            items.push(Item::parse(r)?);
        }

        for _ in 0..header.num_actions {
            // discards comments
            Text::parse(r)?;
        }

        let trailer = Trailer::parse(r)?;

        let data = Data {
            header: header,
            actions: actions,
            verbs: verbs,
            noums: noums,
            rooms: rooms,
            messages: messages,
            items: items,
            trailer: trailer,
        };

        Ok(data)
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or(String::from("None"))
        )
    }
}

impl Data {
    pub fn dump(&self) {
        println!("{}", self.header);
        for i in 0..self.header.num_actions {
            println!("actions[{}]: {}", i, self.actions[i as usize]);
        }

        for i in 0..self.header.num_words {
            println!("verbs[{}]: {}", i, self.verbs[i as usize]);
        }

        for i in 0..self.header.num_words {
            println!("noums[{}]: {}", i, self.noums[i as usize]);
        }

        for i in 0..self.header.num_rooms {
            println!("rooms[{}]: {}", i, self.rooms[i as usize]);
        }

        for i in 0..self.header.num_messages {
            println!("messages[{}]: {}", i, self.messages[i as usize]);
        }

        for i in 0..self.header.num_items {
            println!("items[{}]: {}", i, self.items[i as usize]);
        }

        println!("{}", self.trailer);
    }
}
