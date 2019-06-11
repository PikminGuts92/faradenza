extern crate csv;
extern crate regex;

use std::path::*;
use std::fs::{self, DirEntry};

use regex::*;

mod address;
pub use address::*;


pub fn open_dir(data_dir: &str) {
    // let path = Path::new(data_dir);
    let path_dir = Path::new(data_dir);
    
    /*if path_dir.is_dir() {
        for entry in fs::read_dir(path_dir)? {
            let entry = entry?;
        }
    }*/

    let mut files = Vec::new();

    let callback = |dir: &DirEntry| {
        let file_path = dir.path().to_str().unwrap().to_owned();

        //let res = file_path.ends_with("");

        
        if file_path.ends_with("statewide.csv") {
            files.push(file_path);
        }
    };

    let result = visit_dirs(path_dir, &do_something);

    println!("\r\n");
    println!("Found {0} files!", files.len());
    for f in files {
        println!("{0}", f);
    }

    //return;
}

fn do_something(dir_entry: &DirEntry) {
    println!("{}", &(dir_entry.path().to_str().unwrap()));
}

fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
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

pub fn get_files(dir: String, filter: String) -> Vec<String> {
    let mut files : Vec<String> = Vec::new();
    let path = Path::new(&dir);

    find_files(&path, &mut files).unwrap();

    let regex = Regex::new(&filter).unwrap();


    files.retain(|t| regex.is_match(&t));

    files
}