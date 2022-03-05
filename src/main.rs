mod collection;
mod parser;

use crate::collection::related_tags;
use crate::parser::{parse_dir,ParsedFile};

use clap::{Parser, Subcommand};
use collection::files_with_tags;
use std::env::current_dir;
use std::io;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
enum TagCommand {
    List {
        tags: Vec<String>
    },
    ListFiles {
        tags: Vec<String>,

        #[clap(long)]
        show_title: bool,

        #[clap(long)]
        show_path: bool,
    },
}

#[derive(Subcommand, Debug)]
enum Command {
    Tags {
        #[clap(subcommand)]
        command: TagCommand,
    },
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    dir: Option<PathBuf>,

    #[clap(subcommand)]
    command: Command,
}

/*
fn print_dir(dir: &PathBuf) -> io::Result<()> {
    let files = parse_dir(dir)?;
    for f in files {
        println!("{:?}", f);
    }
    Ok(())
}
*/

fn tag_list_files(
    dir: &PathBuf,
    tags: &Vec<String>,
    show_title: bool,
    show_path: bool,
) -> io::Result<()> {
    let files = parse_dir(dir)?;
    let files_ref = files.iter().collect::<Vec<&ParsedFile>>();
    let result = files_with_tags(&files_ref, &tags);
    for f in result {
        let mut output = Vec::new();
        if show_path || (!show_path && !show_title) {
            output.push(String::from(
                    f.path.to_owned().to_str().unwrap_or("")));
        }
        if show_title {
            let title = f.title.to_owned().unwrap_or(String::from("<no title>"));
            output.push(title);
        }
        println!("{}", output.join("\t"));
    }
    Ok(())
}

fn tag_list(dir: &PathBuf, tag_filter: &Vec<String>) -> io::Result<()> {
    let files = parse_dir(dir)?;
    let files_ref = files.iter().collect::<Vec<&ParsedFile>>();
    let mut tags = related_tags(&files_ref, tag_filter).into_iter().collect::<Vec<_>>();
    tags.sort();
    for t in tags {
        println!("{}", t);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let cur_dir = current_dir()?;
    let dir = args.dir.unwrap_or(cur_dir);

    match args.command {
        Command::Tags { command: cmd } => match cmd {
            TagCommand::List { tags } => tag_list(&dir, &tags),
            TagCommand::ListFiles {
                tags,
                show_title,
                show_path,
            } => tag_list_files(&dir, &tags, show_title, show_path),
        },
    }
}
