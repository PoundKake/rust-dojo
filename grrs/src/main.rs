use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

// fn main() {
fn main() -> std::io::Result<()> {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    //
    // let args = Cli {
    //     pattern,
    //     path: std::path::PathBuf::from(path),
    // };

    // Old Method
    // let args = Cli::parse();
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
    // println!("pattern: {:?}, path {:?}", args.pattern, args.path);

    // Faster Method with BufReader
    let args = Cli::parse();
    let f = File::open(&args.path)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{line}");
        }
    }
    Ok(())
}
