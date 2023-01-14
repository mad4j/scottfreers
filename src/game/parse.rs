use std::io::{BufReader, Error, Read};

pub trait Parse {
    //fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error>
    fn parse(r: &mut BufReader<impl Read>) -> Result<Self, Error>
    where
        Self: Sized;
}
