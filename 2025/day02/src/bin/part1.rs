// AoC 2025 day 02 part 1

use aoc_2025_02::{count_invalid_ids, is_invalid};
use std::fs;

fn solve(data: &str) -> String {
    data.split(",")
        .map(|range| range.split("-").collect::<Vec<&str>>())
        .map(|range| {
            (
                range[0].parse::<i64>().expect("parsing error"),
                range[1].parse::<i64>().expect("parsing error"),
            )
        })
        .map(|(begin, end)| count_invalid_ids(begin, end, &is_invalid))
        .sum::<i64>()
        .to_string()
}

fn main() {
    let filecontent = fs::read_to_string("input.txt").expect("Failed to read input file");

    let result = solve(&filecontent);
    println!("{}", result);

    fs::write("result1.txt", &result).expect("Failed to write result file");
}
