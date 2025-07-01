use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    file: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let args_file = std::fs::read_to_string(&args.file)
        .with_context(|| format!("could not read file `{}`", args.file.display()))?;
    println!("\r{}", args_file);

    Ok(())
}
