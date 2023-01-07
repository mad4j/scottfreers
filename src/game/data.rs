use super::{action::Action, header::Header, parse::Parse, word::Word};

use std::{
    fmt,
    fs::File,
    io::{BufReader, Error},
};

pub struct Data {
    pub header: Header,
    pub actions: Vec<Action>,
    pub verbs: Vec<Word>,
    pub noums: Vec<Word>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            header: Header::new(),
            actions: Vec::new(),
            verbs: Vec::new(),
            noums: Vec::new(),
        }
    }
}

impl Parse for Data {
    fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error> 
        where Self: Sized 
    {
        let mut data = Data::new();

        data.header = Header::parse(r)?;

        for _ in 0..data.header.num_actions {
            let a = Action::parse(r)?;
            data.actions.push(a);
        }

        for _ in 0..data.header.num_words {
            let v = Word::parse(r)?;
            data.verbs.push(v);
            let n = Word::parse(r)?;
            data.noums.push(n);
        }

        Ok(data)
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.header)
    }
}
