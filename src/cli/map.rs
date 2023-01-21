
use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind},
};

use crate::{game::{data::Data, parse::Parse}, games::utils::get_game_data};

use super::args::InfoArgs;


const DIRECTIONS: [&str; 6] = [ "north", "south", "east", "west", "up", "down"];

pub fn cli_map(args: &InfoArgs) -> Result<(), Error> {
    let data = if let Some(path) = &args.file {
        let mut file = File::open(path)?;
        let mut reader = BufReader::new(&mut file);
        Data::parse(&mut reader)?
    } else {
        let game_data = get_game_data(&args.game)
            .ok_or(Error::new(ErrorKind::InvalidData, "Game not found"))?;
        let mut reader = BufReader::new(game_data.as_bytes());
        Data::parse(&mut reader)?
    };

    for i in 0..data.rooms.len() {

        let r = &data.rooms[i];
        println!("r{}: {{ tooltip: {} }}", i, r.text);
    }

    for i in 0..data.rooms.len() {

        let r = &data.rooms[i];

        for k in 0..r.exits.len() {
            if r.exits[k] != 0 {
                println!("r{} -> r{}: {}", i, r.exits[k], DIRECTIONS[k]);
            }
        }
    }

    Ok(())
}