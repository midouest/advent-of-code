use crate::core::Solver;

use cursive::Printer;

pub struct SolvePart1 {
    chargers: Vec<usize>,
    sorted: bool,
    i: usize,
    num_1s: usize,
    num_3s: usize,
}

impl SolvePart1 {
    pub fn new(chargers: Vec<usize>) -> Self {
        Self {
            chargers,
            sorted: false,
            i: 1,
            num_1s: 1,
            num_3s: 1,
        }
    }
}

impl Solver<usize> for SolvePart1 {
    fn is_done(&self) -> bool {
        self.i >= self.chargers.len()
    }

    fn solution(&self) -> Option<usize> {
        self.with_done_some(self.num_1s * self.num_3s)
    }

    fn step(&mut self) {
        if !self.sorted {
            self.chargers.sort();
            self.sorted = true;
            return;
        }

        let prev = self.chargers[self.i - 1];
        let next = self.chargers[self.i];
        let diff = next - prev;

        if diff == 1 {
            self.num_1s += 1;
        } else if diff == 3 {
            self.num_3s += 1;
        }

        self.i += 1;
    }

    fn draw(&self, _printer: &Printer) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::SolvePart1;

    use crate::core::Solver;

    #[test]
    fn it_solves_the_simple_example() {
        let chargers = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let mut solver = SolvePart1::new(chargers);
        assert_eq!(solver.solve(), Some(7 * 5));
    }

    #[test]
    fn it_solves_the_larger_example() {
        let chargers = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let mut solver = SolvePart1::new(chargers);
        assert_eq!(solver.solve(), Some(22 * 10));
    }
}
