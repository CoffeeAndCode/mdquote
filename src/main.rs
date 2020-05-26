#![deny(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic)]

mod lib;

use lib::quotify;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    io::stdout().write_all(quotify(&buffer).as_bytes())?;
    io::stdout().write_all(b"\n")?;

    Ok(())
}
