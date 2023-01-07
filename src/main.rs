mod game;

use game::parse::Parse;

use std::{
    fs::File,
    io::{BufReader, Error},
};

use crate::game::data::Data;

fn main() -> Result<(), Error> {
    let mut file = File::open("adv00")?;
    let mut reader = BufReader::new(&mut file);

    let data = Data::parse(&mut reader)?;

    println!("Game Header");
    println!("{}", data);

    Ok(())
}
