pub mod part1;
pub mod part2;

use std::{cell::RefCell, collections::HashSet, rc::Rc};

use cursive::Cursive;
use part1::Solve2Sum;
use part2::Solve3Sum;

use crate::core::{
    fs::parse_lines,
    puzzle::{Puzzle, PuzzlePart},
    solver::SolverController,
};

#[derive(Debug)]
pub struct Day01 {}

impl Day01 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Puzzle for Day01 {
    fn get_title(&self) -> String {
        "Report Repair".to_string()
    }

    fn is_implemented(&self, _part: PuzzlePart) -> bool {
        true
    }

    fn run(&self, part: PuzzlePart, c: Rc<RefCell<Cursive>>) {
        let elements: Vec<i64> =
            parse_lines("input/day01/expense_report.txt").expect("Could not load puzzle input");
        if part == PuzzlePart::One {
            let solver = Solve2Sum::new(elements, 2020);
            let controller = SolverController::new(solver);
            controller.run(c);
        } else {
            let solver = Solve3Sum::new(elements, 2020);
            let controller = SolverController::new(solver);
            controller.run(c);
        }
    }
}

/// Given a set of integers, find two elements that sum to a given value
pub fn find_2sum(xs: &HashSet<i32>, sum: i32) -> Option<(i32, i32)> {
    xs.iter().find_map(|&x| {
        let y = sum - x;
        if xs.contains(&y) {
            return Some((x, y));
        }
        None
    })
}

/// Given a set of integers, find three elements that sum to a given value
pub fn find_3sum(xs: &HashSet<i32>, sum: i32) -> Option<(i32, i32, i32)> {
    xs.iter().find_map(|&x| {
        let t = sum - x;
        if let Some((y, z)) = find_2sum(xs, t) {
            return Some((x, y, z));
        }
        None
    })
}

#[cfg(test)]
mod tests {
    use super::{find_2sum, find_3sum};
    use std::collections::HashSet;

    fn get_test_set() -> HashSet<i32> {
        [1721, 979, 366, 299, 675, 1456]
            .iter()
            .cloned()
            .collect::<HashSet<_>>()
    }

    #[test]
    fn it_finds_two_numbers_that_sum_to_2020() {
        let xs = get_test_set();
        let pair = find_2sum(&xs, 2020).map(|t| t.0 * t.1);
        assert_eq!(pair, Some(514579));
    }

    #[test]
    fn it_finds_three_numbers_that_sum_to_2020() {
        let xs = get_test_set();
        let pair = find_3sum(&xs, 2020).map(|t| t.0 * t.1 * t.2);
        assert_eq!(pair, Some(241861950));
    }

    #[test]
    fn it_returns_none_if_not_found() {
        let xs = get_test_set();
        let pair = find_2sum(&xs, 2021);
        assert_eq!(pair, None);
    }
}
