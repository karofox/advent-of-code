// AoC 2025 day 1 part 2

use aoc_2025_1::parse_rotation2;
use iter_accumulate::IterAccumulate;
use std::fs;

fn solve(data: &str) -> String {
    data.lines()
        .accumulate((50, 0), |(position, _), line| {
            parse_rotation2(line, position)
        })
        .reduce(|(_, counter), (_, num_0)| (0, counter + num_0))
        .unwrap()
        .1
        .to_string()
}

fn main() {
    let filecontent = fs::read_to_string("input.txt").expect("Failed to read input file");

    let result = solve(&filecontent);
    println!("{}", result);

    fs::write("result2.txt", &result).expect("Failed to write result file");
}
