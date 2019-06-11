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

    let mut address_list = Vec::new();

    // Reads .csv files
    for f in &files {
        let rdr = csv::Reader::from_path(Path::new(f));
        for result in rdr.unwrap().records() {
            let record: csv::StringRecord = result.unwrap();

            let address = get_address_from_record(&record);
            address_list.push(address);
        }

        println!("Found {0} records!", address_list.len());

        break;
    }
}

fn parse_float(s: &str) -> f64 {
    match s.trim() {
        "" => 0.0,
        _ => match s.parse::<f64>() {
            Ok(n) => n,
            Err(_) => 0.0
        },
    }
}

fn get_address_from_record(record: &csv::StringRecord) -> Address {
    Address {
        longitude: match record.get(0) {
            Some(value) => parse_float(value),
            None => 0.0
        },
        latitude: match record.get(1) {
            Some(value) => parse_float(value),
            None => 0.0
        },
        number: match record.get(2) {
            Some(value) => value.to_owned(),
            None => String::new()
        },
        street: match record.get(3) {
            Some(value) => value.to_owned(),
            None => String::new()
        },
        unit: match record.get(4) {
            Some(value) => value.to_owned(),
            None => String::new()
        },
        city: match record.get(4) {
            Some(value) => value.to_owned(),
            None => String::new()
        },
        district: match record.get(5) {
            Some(value) => value.to_owned(),
            None => String::new()
        },
        region: match record.get(6) {
            Some(value) => value.to_owned(),
            None => String::new()
        },
        post_code: match record.get(7) {
            Some(value) => value.to_owned(),
            None => String::new()
        },
        hash: 0,
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