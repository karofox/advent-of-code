// AoC 2025 day 1 part 1

use aoc_2025_1::parse_rotation;
use iter_accumulate::IterAccumulate;
use std::fs;

fn solve(data: &str) -> String {
    data.lines()
        .accumulate(50, |position, line| parse_rotation(line, position))
        .filter(|val| *val == 0)
        .count()
        .to_string()
}

fn main() {
    let filecontent = fs::read_to_string("input.txt").expect("Failed to read input file");

    let result = solve(&filecontent);
    println!("result: {}", result);

    fs::write("result1.txt", &result).expect("Failed to write result file");
}
