mod cli;
mod game;
mod games;

use cli::{
    args::{CliArgs, Commands},
    info::cli_info,
    list::cli_list,
};

use clap::Parser;
use log::{set_max_level, trace, LevelFilter};
use std::io::Error;

use crate::cli::map::cli_map;

fn main() -> Result<(), Error> {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let cli = CliArgs::parse();

    set_max_level(match cli.debug {
        0 => LevelFilter::Off,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    });

    trace!(
        "CLI arguments:\n{}",
        serde_json::to_string_pretty(&cli).unwrap()
    );

    match cli.command {
        Commands::Info(info_args) => cli_info(&info_args),
        Commands::Map(info_args) => cli_map(&info_args),
        Commands::List => Ok(cli_list()),
        Commands::Play { game: _ } => todo!(),
    }
}
