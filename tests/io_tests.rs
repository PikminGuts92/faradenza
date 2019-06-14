#[cfg(test)]
mod tests {
    use faradenza;
    use regex::{Regex};

    #[test]
    fn filter_strings_when_one_matches() {
      // Setup
      let csv_regex = Regex::new(r"(?i)ct\\statewide.csv$").unwrap();

      let mut paths: Vec<String> = vec![
        "D:\\openaddr-collected-us_northeast\\us\\ct\\runme.exe".to_owned(),
        "D:\\openaddr-collected-us_northeast\\us\\ct\\statewide.csv".to_owned(),
        "D:\\openaddr-collected-us_northeast\\us\\ma\\statewide.csv".to_owned(),
      ];

      faradenza::filter_strings(&mut paths, &csv_regex);
      assert_eq!(paths.len(), 1);
    }

    #[test]
    fn calculate_hash_when_same_value_but_different_instance() {
      let a = String::from("test");
      let b = String::from("test");

      let a_hash = faradenza::calculate_hash(&a);
      let b_hash = faradenza::calculate_hash(&b);

      assert_eq!(a_hash, b_hash);
    }

    #[test]
    fn calculate_hash_when_different_casing() {
      let a = String::from("test");
      let b = String::from("Test");

      let a_hash = faradenza::calculate_hash(&a);
      let b_hash = faradenza::calculate_hash(&b);

      assert_eq!(a_hash, b_hash);
    }

    #[test]
    fn calculate_hash_when_different_words() {
      let a = String::from("test");
      let b = String::from("test2");

      let a_hash = faradenza::calculate_hash(&a);
      let b_hash = faradenza::calculate_hash(&b);

      assert_ne!(a_hash, b_hash);
    }
}
