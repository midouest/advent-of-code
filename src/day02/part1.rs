use crate::core::fs::parse_lines;

use super::{OldPolicy, PasswordEntry};

pub fn solve(puzzle_input: &str) {
    print!("Solving 02-1... ");
    let database: Vec<PasswordEntry> =
        parse_lines(puzzle_input).expect("Could not load puzzle input");
    let num_valid = database
        .into_iter()
        .map(|p| p.is_valid::<OldPolicy>())
        .filter(|x| x.is_valid())
        .count();
    println!("solution: {}", num_valid);
}
