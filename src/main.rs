use faradenza;
use regex::{Regex};
use std::env;

fn main() {
	let arguments: Vec<String> = env::args().collect();

	if arguments.len() <= 1 {
		println!("Usage: faradenza.exe D:\\openaddr-collected-us_northeast");
		return;
	}

	// Finds files based on arg path and filter
	let csv_regex = Regex::new(r"(?i)\\statewide.csv$").unwrap();
	let files = faradenza::get_files(&arguments[1], &csv_regex);
	
	match files.len() {
		0 => {
			println!("Found 0 files");
			return;
		},
		1 => {
			println!("Found 1 file");
		},
		len @ _ => {
			println!("Found {} files", len);
		}
	}

	// Consumes and processes input data
	let mut search = faradenza::Search::new();
	println!("\r\nParsing files...");

	search.consume_data(&files);
	println!("Parsed {} total records with {} unique terms", search.get_record_count(), search.get_term_count());

	// Starts server
	println!("\r\nStarting server...");
	faradenza::run_server(search);
}