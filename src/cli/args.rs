use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Dump the content of a game file
    Info(InfoArgs),

    /// Play a game file
    Play {
        /// game file
        #[arg(value_name = "FILE")]
        game: PathBuf,
    },
}
#[derive(Args)]
pub struct InfoArgs {
    /// game file
    #[arg(value_name = "FILE")]
    pub game: PathBuf,

    /// no output, parse game file only
    #[arg(short, long)]
    pub silent: bool,
}
