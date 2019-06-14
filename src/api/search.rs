use crate::api::io;

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
}

pub struct Search {
	record_count: usize,
	terms: Vec<String>,
	term_counts: HashMap<u32, u32>
}

impl Search {
	pub fn new() -> Search {
		Search {
			record_count: 0,
			terms: Vec::new(),
			term_counts: HashMap::new(),
		}
	}

	pub fn consume_data(&mut self, csv_paths: &Vec<String>) {
		let result: Vec<Address> = csv_paths
			.par_iter()
			.flat_map(|f| {
				match io::read_from_csv(f) {
					Ok(value) => value,
					Err(_) => Vec::default(),
				}
			})
			.collect();
		
		// Resets states
		self.terms.clear();
		self.term_counts.clear();

		// Calculates frequency of words in street names
		self.calculate_freqs(&result);
		self.terms.sort();
	}

	pub fn get_frequency(&self, value: &String) -> u32 {
		let hash = io::calculate_hash(value);
		
		match self.term_counts.get(&hash) {
			Some(&freq) => freq,
			None => 0
		}
	}

	pub fn get_terms_copied(&self) -> Vec<String> {
		self.terms.to_vec()
	}

	pub fn get_record_count(&self) -> usize {
		self.record_count
	}

	pub fn get_term_count(&self) -> usize {
		self.term_counts.len()
	}

	fn calculate_freqs(&mut self, data: &Vec<Address>) {
		let mut count = 0;
		
		for a in data {
			let terms = a.street.split(" ");

			for t in terms {
				let lowered = String::from(t).to_lowercase();
				let hash = io::calculate_hash(&lowered);
				
				let freq = match self.term_counts.get(&hash) {
					Some(&freq) => freq + 1,
					None => 1
				};

				if freq == 1 {
					self.terms.push(lowered)
				}

				self.term_counts.insert(hash, freq);
			}
			count += 1;
		}

		self.record_count = count;
	}
}
