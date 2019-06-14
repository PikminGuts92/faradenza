mod api;

pub use api::io::{get_files, read_from_csv};
pub use api::service::{AddressService};
pub use api::search::{Search};

pub fn open_dir(data_dir: &str) {
    let filter = "(?i)vt\\statewide.csv$";
    let files = get_files(data_dir, filter);

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
/*
    let count: usize = files.par_iter()
        .map(|f| {
            match read_from_csv(f) {
                Ok(address_list) => address_list.len(),
                Err(_) => 0,
            }
        })
        .sum();
    

    //read_from_csv(f)

    println!("Found {0} records!", count);*/
}
