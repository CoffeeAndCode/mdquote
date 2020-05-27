#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    unsafe_code
)]

mod lib;

use lib::quotify;
use std::io::{self, Read, Write};
use structopt::StructOpt;

#[derive(StructOpt)]
/// Add Markdown quotes to the start of each line. For example, "hello world"
/// would become "> hello world".
///
///  An example of "quoting" your clipboard (depending on your operating
///  system) would be `pbpaste | mdquote`.
struct Opt {}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    io::stdout().write_all(quotify(&buffer).as_bytes())?;
    io::stdout().write_all(b"\n")?;

    Opt::from_args();
    Ok(())
}
