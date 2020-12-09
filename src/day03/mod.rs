pub mod part1;
pub mod part2;

use std::convert::TryFrom;
use std::{cell::RefCell, rc::Rc};

use cursive::Cursive;
use part1::SolvePart1;
use part2::SolvePart2;

use crate::core::{
    fs::parse_lines,
    grid::Grid,
    puzzle::{Puzzle, PuzzlePart},
    solver::solve,
};

#[derive(Debug)]
pub struct Day03 {}

impl Day03 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Puzzle for Day03 {
    fn get_title(&self) -> String {
        "Toboggan Trajectory".to_string()
    }

    fn is_implemented(&self, _part: PuzzlePart) -> bool {
        true
    }

    fn run(&self, part: PuzzlePart, c: Rc<RefCell<Cursive>>) {
        let rows: Vec<String> =
            parse_lines("input/day03/map.txt").expect("Could not load puzzle input");
        let grid = Grid::try_from(rows).expect("Could not parse puzzle input");
        if part == PuzzlePart::One {
            let solver = SolvePart1::new(grid, (3, 1));
            solve(solver, c);
        } else {
            let strategies = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
            let solver = SolvePart2::new(grid, strategies);
            solve(solver, c);
        }
    }
}
