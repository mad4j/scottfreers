use super::{parse, token::Token};

use std::{
    fmt,
    fs::File,
    io::{BufReader, Error},
};

#[derive(Debug)]
pub struct Action {
    vocab: u16,
    conditions: [Token; 5],
    actions: [Token; 2],
}

impl parse::Parse for Action {
    fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let a = Action {
            vocab: Token::parse(r)?,
            conditions: [
                Token::parse(r)?,
                Token::parse(r)?,
                Token::parse(r)?,
                Token::parse(r)?,
                Token::parse(r)?,
            ],
            actions: [Token::parse(r)?, Token::parse(r)?],
        };

        Ok(a)
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Action {{ vocab: {}, conditions: [{}, {}, {}, {}, {}], actions: [{}, {}]}}",
            self.vocab,
            self.conditions[0],
            self.conditions[1],
            self.conditions[2],
            self.conditions[3],
            self.conditions[4],
            self.actions[0],
            self.actions[1]
        )
    }
}
