mod api;

use api::address_reader;
use api::file_utils;
use rayon::prelude::*;

pub fn open_dir(data_dir: &str) {
    let filter = "(?i)statewide.csv$";
    let files = file_utils::get_files(data_dir, filter);

    println!("\r\n");
    println!("Found {0} files!", files.len());
    for f in &files {
        println!("{0}", f);
    }

    // Reads in .csv files
    /*
    let mut count = 0;
    for f in &files {
        let mut reader = address_reader::AddressReader::new();
        reader.read_from_csv(f);

        count += reader.len();
    }*/

    let count: usize = files.par_iter()
        .map(|f| {
            let mut reader = address_reader::AddressReader::new();
            reader.read_from_csv(f);
            reader.len()
        })
        .sum();

    println!("Found {0} records!", count);
}
