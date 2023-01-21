use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Parser, Debug, Serialize)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Serialize)]
pub enum Commands {
    /// Dump the content of a game file
    Info(InfoArgs),

    /// Display game map
    Map(InfoArgs),

    /// Play a game file
    Play {
        /// game file
        #[arg(value_name = "FILE")]
        game: PathBuf,
    },
}
#[derive(Args, Debug, Serialize)]
pub struct InfoArgs {
    /// game file
    #[arg(short, long, value_name = "FILE", group = "game_source")]
    pub file: Option<PathBuf>,

    /// game name
    #[arg(
        short,
        long,
        value_name = "NAME",
        default_value = "adventureland",
        group = "game_source"
    )]
    pub game: String,

    /// dump addressed sections
    #[arg(short, long, value_enum, default_value = "all")]
    pub sections: Section,

    #[arg(short, long, value_enum, default_value = "plain")]
    pub output_type: OutputType,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize)]
pub enum OutputType {
    /// output information as plain text
    Plain,

    /// output information as json data
    Json,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize)]
pub enum Section {
    /// silent parsing, no output
    None,

    /// output Header section
    Header,

    /// output Actions section
    Actions,

    /// output Verbs section
    Verbs,

    /// output Noums section
    Noums,

    /// output Rooms section
    Rooms,

    /// output Messages section
    Messages,

    /// output Items section
    Items,

    /// output Trailer section
    Trailer,

    /// output all sections
    All,
}
