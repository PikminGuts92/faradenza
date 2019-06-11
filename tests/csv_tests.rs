//use faradenza::*;

use faradenza;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_1() {
      assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_open_dir() {
      /*
      let path = "D:\\openaddr-collected-us_northeast";
      let filter = "(?i)statewide.csv$";

      let result = faradenza::get_files(path, filter);

      for r in result {
        println!("{}", r);
      }*/

      faradenza::open_dir("D:\\openaddr-collected-us_northeast");
    }
}
