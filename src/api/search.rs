use crate::api::io::read_from_csv;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct Address {
  pub longitude: f64,
  pub latitude: f64,
  pub number: String,
  pub street: String,
  pub unit: String,
  pub city: String,
  pub district: String,
  pub region: String,
  pub post_code: String,
  pub hash: u64
}

pub struct Search {

}

impl Search {
	pub fn new() -> Search {
		Search {

		}
	}

	pub fn consume_data(&self, csv_paths: &Vec<String>) {
		let result: Vec<Address> = csv_paths.par_iter()
			.flat_map(|f| {
				match read_from_csv(f) {
					Ok(value) => value,
					Err(_) => Vec::default(),
				}
			})
			.collect();
		
	}

	pub fn process_data(&self) {

	}

	pub fn get_data(&self) -> String {
		String::from("Whatever dude")
	}
}