use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

pub trait Parse {
    fn parse(r: &mut BufReader<&mut File>) -> Result<Self, Error>
    where
        Self: Sized;

    fn parse_u16(r: &mut BufReader<&mut File>) -> Result<u16, Error> {
        let mut l = String::new();
        r.read_line(&mut l)?;

        let v = l
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        Ok(v)
    }
}
