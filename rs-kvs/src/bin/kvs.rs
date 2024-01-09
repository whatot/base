use std::process;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(_) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        _ => {
            unreachable!()
        }
    }
}
