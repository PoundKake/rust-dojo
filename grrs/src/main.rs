use anyhow::{Context, Result};
use clap::Parser;
// use std::fs::File;
// use std::io::BufReader;
// use std::io::prelude::*;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}

fn answer() -> i32 {
    return 42;
}
// #[derive(Debug)]
// struct CustomError(String);

fn main() -> Result<()> {
    // fn main() {
    // fn main() -> Result<(), CustomError> {
    // fn main() -> std::io::Result<()> {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // let args = Cli {
    //     pattern,
    //     path: std::path::PathBuf::from(path),
    // };

    // Old Method
    let args = Cli::parse();
    // let result = std::fs::read_to_string(&args.path);
    // let content = match result {
    //     Ok(content) => {
    //         content;
    //     }
    //     Err(error) => {
    //         panic!("Can't deal with {}, just exit here!", error);
    //     }
    // };

    // let result = std::fs::read_to_string(&args.path);
    // let content = match result {
    //     Ok(content) => {content;}
    //     Err(error) => {return Err(error.into());}
    // };

    // let content = std::fs::read_to_string(&args.path)?;
    // println!("file contents: {}", content);
    // Ok(())

    // let content = std::fs::read_to_string(&args.path)
    //     .map_err(|err| CustomError(format!("Error reading `{:?}`: {:?}", &args.path, err)))?;
    // println!("file contents: {}", content);
    // Ok(())

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let _ = grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    // let pb = indicatif::ProgressBar::new(100);
    // for i in 0..100 {
    //     // pb.println(format!("[+] finshed #{}", i));
    //     pb.inc(1);
    // }
    // pb.finish_with_message("done");

    Ok(())

    // println!("pattern: {:?}, path {:?}", args.pattern, args.path);

    // Faster Method with BufReader
    // let args = Cli::parse();
    // let f = File::open(&args.path)?;
    // let f = BufReader::new(f);
    //
    // for line in f.lines() {
    //     let line = line?;
    //     if line.contains(&args.pattern) {
    //         println!("{line}");
    //     }
    // }
    // Ok(())
}
