mod cli;
mod game;

use cli::{
    args::{CliArgs, Commands},
    info::cli_info,
};

use clap::Parser;
use log::{set_max_level, LevelFilter};
use std::io::Error;

fn main() -> Result<(), Error> {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let cli = CliArgs::parse();

    set_max_level(match cli.debug {
        0 => LevelFilter::Off,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    }
    );

    match cli.command {
        Commands::Info(info_args) => cli_info(info_args),
        Commands::Play { game: _ } => todo!(),
    }
}
