use std::collections::HashMap;

use cursive::{
    theme::{ColorStyle, Style},
    utils::span::SpannedStr,
    utils::span::SpannedString,
    Printer,
};

use crate::core::{grid::Grid, solver::Solver, util::last_n, wrap_grid::WrapGrid};

pub struct SolvePart1 {
    grid: WrapGrid<char>,
    strategy: (usize, usize),
    position: (usize, usize),
    count: i64,
    history: HashMap<usize, (usize, char)>,
}

impl SolvePart1 {
    pub fn new(grid: Grid<char>, strategy: (usize, usize)) -> Self {
        Self {
            grid: WrapGrid::new(grid),
            strategy,
            position: (0, 0),
            count: 0,
            history: HashMap::new(),
        }
    }
}

impl Solver for SolvePart1 {
    fn is_done(&self) -> bool {
        self.position.1 >= self.grid.height()
    }

    fn solution(&self) -> Option<i64> {
        self.with_done_some(self.count)
    }

    fn step(&mut self) {
        if self.is_done() {
            return;
        }

        let mut x = self.position.0;
        let mut y = self.position.1;
        let current = self.grid.get(x, y).unwrap();

        let marker: char;
        if current == &'#' {
            self.count += 1;
            marker = 'X';
        } else {
            marker = 'O';
        }
        self.history.insert(y, (x, marker));

        x += self.strategy.0;
        y += self.strategy.1;

        self.position = (x, y);
    }

    fn draw(&self, printer: &Printer) {
        let y_range = last_n(self.position.1.min(self.grid.height()), printer.size.y);
        for (i, y) in y_range.enumerate() {
            let x_range = last_n(self.position.0, printer.size.x);
            let text: String = x_range.map(|x| self.grid.get(x, y).unwrap()).collect();
            let mut styled: SpannedString<Style>;
            if let Some((x, c)) = self.history.get(&y) {
                let x = x % printer.size.x;
                let (plain1, rest) = text.split_at(x);
                let (_, plain2) = rest.split_at(1);
                styled = SpannedString::<Style>::plain(plain1);
                let color = if c == &'X' {
                    ColorStyle::secondary()
                } else {
                    ColorStyle::tertiary()
                };
                styled.append_styled(c.to_string(), Style::from(color));
                styled.append_plain(plain2);
            } else {
                styled = SpannedString::<Style>::plain(text);
            }
            printer.print_styled((0, i), SpannedStr::from(&styled));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::core::{grid::Grid, solver::Solver};

    use super::SolvePart1;

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

        let mut solver = SolvePart1::new(grid, (3, 1));
        let solution = solver.solve();

        assert_eq!(solution, Some(7));
    }
}
