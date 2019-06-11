//use faradenza::*;

use faradenza;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_1() {
      let add = faradenza::Address {
        longitude: 0.0,
        latitude: 0.0,
        number: String::from(""),
        street: String::from(""),
        unit: String::from(""),
        city: String::from(""),
        district: String::from(""),
        region: String::from(""),
        post_code: String::from(""),
        hash: 0,
      };

      assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_open_dir() {
      let path = String::from("D:\\openaddr-collected-us_northeast");
      let filter = String::from("(?i)statewide.csv$");

      let result = faradenza::get_files(path, filter);

      for r in result {
        println!("{}", r);
      }
    }
}
