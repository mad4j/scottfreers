use super::parse;

use std::{
    fmt,
    fs::File,
    io::{BufReader, Error},
};

#[derive(Debug)]
pub struct Action {
    vocab: u16,
    conditions: [u16; 5],
    actions: [u16; 2],
}

impl Action {
    pub fn new() -> Action {
        Action {
            vocab: 0,
            conditions: [0; 5],
            actions: [0, 2],
        }
    }

    pub fn init(vocab: u16, conditions: [u16; 5], actions: [u16; 2]) -> Action {
        Action {
            vocab,
            conditions,
            actions,
        }
    }
}

impl parse::Parse for Action {
    fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error> 
        where Self: Sized
    {
        let a = Action {
            vocab: Self::parse_u16(r)?,
            conditions: [
                Self::parse_u16(r)?,
                Self::parse_u16(r)?,
                Self::parse_u16(r)?,
                Self::parse_u16(r)?,
                Self::parse_u16(r)?,
            ],
            actions: [
                Self::parse_u16(r)?,
                Self::parse_u16(r)?,
            ],
        };

        Ok(a)
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = format!("Vocab       : {}\n", self.vocab);
        let c0 = format!("Condition[0]: {}\n", self.conditions[0]);
        let c1 = format!("Condition[1]: {}\n", self.conditions[1]);
        let c2 = format!("Condition[2]: {}\n", self.conditions[0]);
        let c3 = format!("Condition[3]: {}\n", self.conditions[1]);
        let c4 = format!("Condition[4]: {}\n", self.conditions[1]);
        let a0 = format!("Actions[0]  : {}\n", self.actions[0]);
        let a1 = format!("Actions[1]  : {}\n", self.actions[1]);

        write!(f, "{}{}{}{}{}{}{}{}", v, c0, c1, c2, c3, c4, a0, a1)
    }
}
