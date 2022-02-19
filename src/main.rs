mod parser;

use crate::parser::{
    parse_dir
};

use clap::{Parser, Subcommand};

use std::path::PathBuf;

#[derive(Subcommand, Debug)]
enum TagCommand {
    List
}

#[derive(Subcommand, Debug)]
enum Commands {
    Tags {
        #[clap(subcommand)]
        tag_command: TagCommand
    }
}

#[derive(Parser, Debug)]
struct Args {
    dir: Option<String>,

    #[clap(subcommand)]
    command: Commands
    
}

fn print_dir(dir: &PathBuf) -> io::Result<()> {
    let files = parse_dir(dir)?;
    for f in files {
        println!("{:?}", f);
    }
    Ok(())
}

fn main() {
    let dir = "/home/leo/zettel";
    print_dir(&PathBuf::from(dir));
}
