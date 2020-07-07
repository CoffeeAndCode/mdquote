#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    unsafe_code
)]

use argh::FromArgs;
use mdquote::add_quotes;
use std::io;
use std::process::exit;

fn version() -> String {
    format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[derive(FromArgs)]
#[argh(
    description = "Add Markdown quotes to the start of each line. For example, \"hello world\" would become \"> hello world\"",
    example = "On a Mac, quote your clipboard contents:\n$ pbpaste | {command_name}"
)]
struct Opt {
    /// prints version information
    #[argh(switch, short = 'V')]
    version: bool,
}

fn main() -> io::Result<()> {
    let args: Opt = argh::from_env();
    if args.version {
        println!("{}", version());
        exit(0);
    }

    let stdin = io::stdin();
    let stdout = io::stdout();

    add_quotes(stdin.lock(), stdout.lock(), true)?;

    Ok(())
}
