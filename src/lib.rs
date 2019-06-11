extern crate csv;
extern crate regex;

use std::path::*;
use std::fs::{self};

use regex::*;

mod address;
pub use address::*;


pub fn open_dir(data_dir: &str) {
    let filter = "(?i)statewide.csv$";
    let files = get_files(data_dir, filter);

    println!("\r\n");
    println!("Found {0} files!", files.len());
    for f in &files {
        println!("{0}", f);
    }

    // Reads .csv files
    for f in &files {
        let mut rdr = csv::Reader::from_path(Path::new(f));
        for result in rdr.unwrap().records() {
            let record = result.unwrap();
            println!("{:?}", record);
            break;
        }

        break;
    }
}

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