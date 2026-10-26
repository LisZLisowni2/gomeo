use clap::Parser;
use gomeo::compile;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "gomeoc")]
#[command(version, about = "The Gomeo compiler")]
struct Args {
    /// Gomeo source file
    input: PathBuf,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = compile(&args.input) {
        eprintln!("[ERROR] Compile failed: {}", e);
    }
}
