/// Operations on collections of ParsedFiles
use crate::parser::ParsedFile;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn all_tags<'a>(files: &[&ParsedFile]) -> HashSet<String> {
    let tags = files
        .iter()
        .flat_map(|f| &f.tags)
        .map(|t| t.clone());
    HashSet::from_iter(tags)
}


pub fn files_with_tags<'a>(files: &'a [&ParsedFile], tags: &[String]) -> Vec<&'a ParsedFile> {
    files
        .iter()
        .map(|f| *f)
        .filter(|f| tags.iter().all(|t| f.tags.contains(t)))
        .collect::<Vec<&ParsedFile>>()
}


pub fn related_tags<'a>(files: &'a [&ParsedFile], tags: &'a [String]) -> HashSet<String> {
    let f = files_with_tags(files, tags);
    all_tags(&f)
}

