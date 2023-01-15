use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind},
};

use super::args::{InfoArgs, Section};
use crate::games::utils::get_game_data;
use crate::{
    cli::args::OutputType,
    game::{data::Data, parse::Parse},
};

pub fn cli_info(args: &InfoArgs) -> Result<(), Error> {
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

    if args.sections == Section::All {
        println!("help {}", data);
    } else {
        dump_sections(&args, &data);
    }

    Ok(())
}

fn dump_sections(args: &InfoArgs, data: &Data) -> () {
    if args.sections == Section::Header {
        match args.output_type {
            OutputType::Plain => {
                println!("{}", data.header);
            }
            OutputType::Json => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&data.header)
                        .unwrap_or(String::from("{{ \"None\" }}"))
                );
            }
        };
    }

    if args.sections == Section::Items {
        match args.output_type {
            OutputType::Plain => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&data.items)
                        .unwrap_or(String::from("{{ \"None\" }}"))
                );
            }
            OutputType::Json => {
                for i in 0..data.items.len() {
                    println!("{}", data.items[i]);
                }
            }
        };
    }

    if args.sections == Section::Trailer {
        match args.output_type {
            OutputType::Plain => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&data.trailer)
                        .unwrap_or(String::from("{{ \"None\" }}"))
                );
            }
            OutputType::Json => {
                for _ in 0..data.items.len() {
                    println!("{}", data.trailer);
                }
            }
        };
    }
}
