// AoC 2025 day 02 library

use pcre2::bytes::Regex;

pub fn count_invalid_ids(begin: i64, end: i64, invalid: &dyn Fn(&str) -> bool) -> i64 {
    (begin..end + 1)
        .map(|number| number.to_string())
        .filter(|id| invalid(id))
        .map(|invalid_id| invalid_id.parse::<i64>().unwrap())
        .sum()
}

pub fn is_invalid(id: &str) -> bool {
    let half = id.len() / 2;
    id[..half] == id[half..]
}

pub fn is_invalid2(id: &str) -> bool {
    let invalid_pattern = Regex::new(r"^(\d+)\1+$").unwrap();
    invalid_pattern.is_match(id.as_bytes()).unwrap()
}
