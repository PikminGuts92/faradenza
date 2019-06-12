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

        

        //break;
    }

    println!("Found {0} records!", address_list.len());
}

fn get_float_record(record: &csv::StringRecord, idx: usize) -> f64 {
    match record.get(idx) {
        Some(value) => match value.trim() {
            "" => 0.0,
            _ => match value.parse::<f64>() {
                Ok(n) => n,
                Err(_) => 0.0
            },
        },
        None => 0.0
    }
}

fn get_string_record(record: &csv::StringRecord, idx: usize) -> String {
    match record.get(idx) {
        Some(value) => value.to_owned(),
        None => String::new()
    }
}

fn get_address_from_record(record: &csv::StringRecord) -> Address {
    Address {
        longitude: get_float_record(record, 0),
        latitude: get_float_record(record, 1),
        number: get_string_record(record, 2),
        street: get_string_record(record, 3),
        unit: get_string_record(record, 4),
        city: get_string_record(record, 5),
        district: get_string_record(record, 6),
        region: get_string_record(record, 7),
        post_code: get_string_record(record, 8),
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