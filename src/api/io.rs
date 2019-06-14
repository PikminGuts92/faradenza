extern crate regex;

use crate::api::search::Address;
use csv;
use regex::*;
use std::fs::{self};
use std::io;
use std::path::*;

pub fn read_from_csv(file_path: &String) -> io::Result<Vec<Address>> {
    let mut address_list = Vec::new();

    let rdr = csv::Reader::from_path(Path::new(file_path));
    for result in rdr?.records() {
        let record: csv::StringRecord = result?;

        // Converts to address struct and appends to collection
        let address = get_address_from_record(&record);
        address_list.push(address);
    }
    Ok(address_list)
}

pub fn get_files(dir: &str, filter: &Regex) -> Vec<String> {
    let mut files : Vec<String> = Vec::new();
    let path = Path::new(&dir);

    // Gets all files (including in sub directories)
    find_files(&path, &mut files).unwrap();

    // Filters files based on regex
    filter_strings(&mut files, &filter);
    //files.retain(|t| regex.is_match(&t));
    files
}

pub fn filter_strings(files: &mut Vec<String>, regex: &Regex) {
    files.retain(|x| regex.is_match(&x));
}

fn find_files(dir: &Path, files: &mut Vec<String>) -> std::io::Result<()> {
    if !dir.is_dir() {
        return Ok(())
    }

    // Recursively finds files and appends path to list
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
