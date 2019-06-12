mod api;

use api::address_reader;
use api::file_utils;


pub fn open_dir(data_dir: &str) {
    let filter = "(?i)statewide.csv$";
    let files = file_utils::get_files(data_dir, filter);

    println!("\r\n");
    println!("Found {0} files!", files.len());
    for f in &files {
        println!("{0}", f);
    }

    let mut reader = address_reader::AddressReader::new();

    // Reads in .csv files
    for f in &files {
        reader.read_from_csv(f);
    }

    println!("Found {0} records!", reader.len());
}
