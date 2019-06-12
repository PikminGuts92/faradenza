extern crate csv;

use crate::api::address::*;
use std::path::*;

pub struct AddressReader {
    address_list: Vec<Address>,
}

impl AddressReader {
    pub fn new() -> AddressReader {
        AddressReader {
            address_list: Vec::new(),
        }
    }

    pub fn read_from_csv(&mut self, filePath: &String) {
        let rdr = csv::Reader::from_path(Path::new(filePath));

        for result in rdr.unwrap().records() {
            let record: csv::StringRecord = result.unwrap();

            let address = get_address_from_record(&record);
            self.address_list.push(address);
        }
    }

    pub fn len(&self) -> usize {
        self.address_list.len()
    }
}

pub fn get_float_record(record: &csv::StringRecord, idx: usize) -> f64 {
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

pub fn get_string_record(record: &csv::StringRecord, idx: usize) -> String {
    match record.get(idx) {
        Some(value) => value.to_owned(),
        None => String::new()
    }
}

pub fn get_address_from_record(record: &csv::StringRecord) -> Address {
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
