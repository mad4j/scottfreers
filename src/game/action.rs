use serde::Serialize;

use super::{parse, token::Token};

use std::{
    fmt,
    io::{BufReader, Error, Read},
};

#[derive(Debug, Serialize)]
pub struct Action {
    verb: Token,
    noum: Token,
    conditions: [Token; 5],
    params: [Token; 5],
    actions: [Token; 4],
}

impl parse::Parse for Action {
    fn parse(r: &mut BufReader<impl Read>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let vocab = Token::parse(r)?;

        let c0 = Token::parse(r)?;
        let c1 = Token::parse(r)?;
        let c2 = Token::parse(r)?;
        let c3 = Token::parse(r)?;
        let c4 = Token::parse(r)?;

        let a0 = Token::parse(r)?;
        let a1 = Token::parse(r)?;

        let a = Action {
            verb: vocab / 150,
            noum: vocab % 150,
            conditions: [c0 % 20, c1 % 20, c2 % 20, c3 % 20, c4 % 20],
            params: [c0 / 20, c1 / 20, c2 / 20, c3 / 20, c4 / 20],
            actions: [a0 / 150, a0 % 150, a1 / 150, a1 % 150],
        };

        Ok(a)
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "action {}",
            serde_json::to_string(self).unwrap_or(String::from("None"))
        )
    }
}
