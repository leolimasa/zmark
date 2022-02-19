use std::fs;
use std::path::PathBuf;
use std::io::{
    BufRead,
    BufReader
};
use std::io;
use lazy_static::lazy_static;
use regex::Regex;


#[derive(Debug)]
pub struct ParsedFile {
    pub title: Option<String>,
    pub path: PathBuf,
    pub tags: Vec<String>,
}

/// Detects any hash tags in the provided line.
fn detect_tags(line: &str) -> Vec<String> {
    /* The lazy_static is a macro library that prevents this
     * regular expression from being compiled more than once
     * if this function is called several times.
     */
    lazy_static! {
        static ref TAG_RE:Regex = Regex::new(r"#[a-z0-9_]+").unwrap();
    }
    let mut tags = Vec::new();
    for tag in TAG_RE.find_iter(line) {
        tags.push(tag.as_str().to_string());
    }
    tags
}

fn detect_title(line: &str) -> Option<String> {
    lazy_static! {
        static ref TITLE_RE:Regex = Regex::new(r"^# (.*)$").unwrap();
    }
    let captures = TITLE_RE.captures(line)?;  
    let title = &captures[1];
    Some(title.to_string())
}

fn parse_file(path: &PathBuf) -> io::Result<ParsedFile> {
    let f = fs::File::open(path)?;
    let reader = BufReader::new(f);
    let mut tags = Vec::new();
    let mut title:Option<String> = None;
    for line_or_err in reader.lines() {
        let line = line_or_err?;
        for tag in detect_tags(&line) {
            tags.push(tag);
        }
        if title == None {
            title = detect_title(&line);
        }
    }
    return Ok(ParsedFile {
        title,
        path: path.clone(),
        tags,
    });
}

/// Returns the file extension for a directory entry. None if it's not
/// a file.
pub fn parse_dir(dir: &PathBuf) -> io::Result<Vec<ParsedFile>> {
    let mut result = Vec::new();
    let entries = fs::read_dir(dir)?;
    for entry_or_err in entries {
        let entry = entry_or_err?;
        let file_type = entry.file_type()?;
        let path = entry.path();
        if file_type.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    let parsed_file = parse_file(&path)?;
                    result.push(parsed_file);
                }
            }
        } else if file_type.is_dir() {
            let subdir_entries = parse_dir(&path)?;
            for parsed in subdir_entries {
                result.push(parsed);
            }
        }
    }
    Ok(result)
}
