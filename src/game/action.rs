use serde::Serialize;

use super::{parse, token::Token};

use std::{
    fmt,
    io::{BufReader, Error, Read},
};

#[derive(Debug, Serialize)]
pub struct Action {
    vocab: Token,
    conditions: [Token; 5],
    actions: [Token; 2],
}

impl parse::Parse for Action {
    fn parse(r: &mut BufReader<impl Read>) -> Result<Self, Error>
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
            actions: [
                Token::parse(r)?, 
                Token::parse(r)?
            ],
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
