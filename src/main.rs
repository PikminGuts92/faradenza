use faradenza;
use regex::{Regex};
use std::env;

fn main() {
	let arguments: Vec<String> = env::args().collect();
	for a in &arguments {
		println!("arg: {}", a);
	}

	if arguments.len() <= 1 {
		println!("Usage: faradenza.exe D:\\openaddr-collected-us_northeast");
		return;
	}

	// Finds files based on arg path and filter
	let csv_regex = Regex::new(r"(?i)\\vt\\statewide.csv$").unwrap();
	let files = faradenza::get_files(&arguments[1], &csv_regex);
	
	if files.len() <= 0 {
		println!("Found 0 files... can't do anything");
		return;
	} else {
		println!("Found {0} files!", files.len());
	}

	match files.len() {
		0 => {
			print!("Found 0 files");
			return;
		},
		1 => {
			print!("Found 1 file");
		},
		len @ _ => {
			println!("Found {} files", len);
		}
	}

	// Consumes and processes input data
	let mut search = faradenza::Search::new();
	search.consume_data(&files);
	search.process_data();

	// Starts server
	faradenza::run_server(search);
}