mod game;

use game::parse::Parse;
use game::data::Data;

use std::{
    fs::File,
    io::{BufReader, Error},
};

use log::{info, trace, warn};

fn main() -> Result<(), Error> {

    simple_logger::SimpleLogger::new().env().init().unwrap();
    info!("ScottFreeRS");

    let mut file = File::open("adv00")?;
    let mut reader = BufReader::new(&mut file);

    let data = Data::parse(&mut reader)?;

    data.dump();

    Ok(())
}
