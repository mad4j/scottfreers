use super::parse;

use std::{
    fs::File,
    io::{BufReader, Error, BufRead},
};

pub type Word = String;

impl parse::Parse for Word {
    fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error>
        where Self: Sized 
    {
        let mut s = String::new();
        r.read_line(&mut s)?;

        Ok(s)
    }
}