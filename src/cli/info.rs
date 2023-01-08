use std::{
    fs::File,
    io::{BufReader, Error},
};

use super::args::InfoArgs;
use crate::game::{data::Data, parse::Parse};

pub fn cli_info(args: InfoArgs) -> Result<(), Error> {
    let mut file = File::open(args.game)?;
    let mut reader = BufReader::new(&mut file);

    let data = Data::parse(&mut reader)?;

    if !args.silent {
        data.dump();
    }

    Ok(())
}
