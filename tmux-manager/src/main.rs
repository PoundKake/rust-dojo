use anyhow::Result;
use clap::Parser;
// use std::cmp::Ordering;
// use std::io;

// use clap to parse cli arguments
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    session_name: String,
    // The path to the file to read
    session_path: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    println!(
        "name is {:?}\npath is {:?}",
        &args.session_name, &args.session_path,
    );
    Ok(())
}
