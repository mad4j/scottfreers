mod game;

use game::data::Data;
use game::parse::Parse;

use std::{
    fs::File,
    io::{BufReader, Error},
};

fn main() -> Result<(), Error> {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let mut file = File::open("adv00")?;
    let mut reader = BufReader::new(&mut file);

    let data = Data::parse(&mut reader)?;

    data.dump();

    Ok(())
}
