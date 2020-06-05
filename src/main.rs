#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    unsafe_code
)]

use clap::Clap;
use mdquote::add_quotes;
use std::io;

/// Add Markdown quotes to the start of each line. For example, "hello world"
/// would become "> hello world".
///
/// An example of "quoting" your clipboard (depending on your operating
/// system) would be `pbpaste | mdquote`.
#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
struct Opt {}

fn main() -> io::Result<()> {
    Opt::parse();

    let stdin = io::stdin();
    let stdout = io::stdout();

    add_quotes(stdin.lock(), stdout.lock(), true)?;

    Ok(())
}
