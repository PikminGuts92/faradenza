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
}
