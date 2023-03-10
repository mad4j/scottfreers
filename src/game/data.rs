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
            header,
            actions,
            verbs,
            noums,
            rooms,
            messages,
            items,
            trailer,
        };

        Ok(data)
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or(String::from("None"))
        )
    }
}
