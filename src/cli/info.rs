use std::{
    fs::File,
    io::{BufReader, Error},
};

use super::args::{InfoArgs, Section};
use crate::{
    cli::args::OutputType,
    game::{data::Data, parse::Parse},
};

pub fn cli_info(args: InfoArgs) -> Result<(), Error> {
    let mut file = File::open(args.game)?;
    let mut reader = BufReader::new(&mut file);

    let data = Data::parse(&mut reader)?;

    if args.sections == Section::Header || args.sections == Section::All {
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

    if args.sections == Section::Items || args.sections == Section::All {
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

    if args.sections == Section::Trailer || args.sections == Section::All {
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

    Ok(())
}
