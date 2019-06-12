extern crate regex;

use regex::*;
use std::fs::{self};
use std::path::*;

fn find_files(dir: &Path, files: &mut Vec<String>) -> std::io::Result<()> {
    if !dir.is_dir() {
        return Ok(())
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            find_files(&path, files).unwrap();
        } else {
            files.push(path.to_str().unwrap().to_string());
        }
    }

    Ok(())
}

pub fn get_files(dir: &str, filter: &str) -> Vec<String> {
    let mut files : Vec<String> = Vec::new();
    let path = Path::new(&dir);

    find_files(&path, &mut files).unwrap();

    let regex = Regex::new(&filter).unwrap();


    files.retain(|t| regex.is_match(&t));

    files
}
