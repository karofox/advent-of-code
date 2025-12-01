// AoC 2025 day 1 library

pub fn parse_rotation(rotation: &str, position: i32) -> i32 {
    match rotation.chars().nth(0).unwrap() {
        'R' => (position + rotation.get(1..).unwrap().parse::<i32>().unwrap()).rem_euclid(100),
        'L' => (position - rotation.get(1..).unwrap().parse::<i32>().unwrap()).rem_euclid(100),
        _ => position,
    }
}

pub fn parse_rotation2(rotation: &str, position: i32) -> (i32, i32) {
    match rotation.chars().nth(0).unwrap() {
        'R' => (
            (position + rotation.get(1..).unwrap().parse::<i32>().unwrap()).rem_euclid(100),
            (position + rotation.get(1..).unwrap().parse::<i32>().unwrap()) / 100,
        ),
        'L' => (
            (position - rotation.get(1..).unwrap().parse::<i32>().unwrap()).rem_euclid(100),
            -(position - rotation.get(1..).unwrap().parse::<i32>().unwrap()).div_euclid(100),
        ),
        _ => (position, 0),
    }
}
