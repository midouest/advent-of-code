use crate::core::{grid::Grid, solver::Solver};
use cursive::Printer;

use super::part1::SolvePart1;

pub struct SolvePart2 {
    grid: Grid<char>,
    solver: Option<SolvePart1>,
    i: usize,
    counts: Vec<i64>,
    strategies: Vec<(usize, usize)>,
}

impl SolvePart2 {
    pub fn new(grid: Grid<char>, strategies: Vec<(usize, usize)>) -> Self {
        Self {
            grid,
            solver: None,
            i: 0,
            counts: Vec::new(),
            strategies,
        }
    }
}

impl Solver for SolvePart2 {
    fn is_done(&self) -> bool {
        self.counts.len() == self.strategies.len()
    }

    fn solution(&self) -> Option<i64> {
        self.with_done_some(self.counts.iter().fold(1, |acc, c| acc * c))
    }

    fn step(&mut self) {
        let strategy = self.strategies[self.i];
        let mut solver = SolvePart1::new(self.grid.clone(), strategy);
        if let Some(count) = solver.solve() {
            self.counts.push(count);
        }
        self.i += 1;
        self.solver = Some(solver);
    }

    fn draw(&self, printer: &Printer) {
        if let Some(solver) = &self.solver {
            solver.draw(printer);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::grid::Grid;
    use crate::core::solver::Solver;
    use std::convert::TryFrom;

    use super::SolvePart2;

    #[test]
    fn it_solves_the_example() {
        let input = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];
        let grid = Grid::try_from(input).unwrap();
        let strategies = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        let mut solver = SolvePart2::new(grid, strategies);
        let solution = solver.solve();

        assert_eq!(solution, Some(336));
    }
}
