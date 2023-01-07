use std::{
    fs::File,
    io::{BufReader, Error},
};

pub trait Parse {
    fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error>
    where
        Self: Sized;
}
