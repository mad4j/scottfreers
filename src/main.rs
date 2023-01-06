use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind}, fmt,
};


mod game_header;

use game_header::GameHeader;

fn main() -> Result<(), Error> {

    let mut file = File::open("adv00")?;
    let gm = GameHeader::parse(&mut file)?;

    println!("{}", gm);

    Ok(())
}
