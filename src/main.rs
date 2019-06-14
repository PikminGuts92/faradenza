use std::env;
use faradenza;

fn main() {
	let arguments: Vec<String> = env::args().collect();

	if arguments.len() <= 1 {
		println!("Usage: faradenza.exe D:\\openaddr-collected-us_northeast");
		return;
	}

	// Finds files based on arg path and filter
	let files = faradenza::get_files(&arguments[1], "(?i)vt\\statewide.csv$");
	if files.len() <= 0 {
		println!("Found 0 files... can't do anything");
		return;
	} else {
		println!("Found {0} files!", files.len());
	}

	// Consumes and processes input data
	let mut search = faradenza::Search::new();
	search.consume_data(&files);
	search.process_data();


	faradenza::open_dir(&arguments[1]);


	println!("Server started!");
}