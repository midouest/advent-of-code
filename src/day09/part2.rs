use crate::core::Solver;

use cursive::Printer;

pub struct SolvePart2 {
    xmas: Vec<usize>,
    target: usize,
    i: usize,
    found: Option<(usize, usize)>,
}

impl SolvePart2 {
    pub fn new(xmas: Vec<usize>, target: usize) -> Self {
        Self {
            xmas,
            target,
            i: 0,
            found: None,
        }
    }
}

impl Solver<usize> for SolvePart2 {
    fn is_done(&self) -> bool {
        self.found.is_some() || self.i >= self.xmas.len()
    }

    fn solution(&self) -> Option<usize> {
        self.found.map(|(l, h)| l + h)
    }

    fn step(&mut self) {
        let mut sum = self.xmas[self.i];

        let mut low = sum;
        let mut high = sum;

        for j in self.i + 1..self.xmas.len() {
            let x = self.xmas[j];
            low = low.min(x);
            high = high.max(x);
            sum += x;

            if sum == self.target {
                self.found = Some((low, high));
                return;
            }

            if sum > self.target {
                break;
            }
        }

        self.i += 1;
    }

    fn draw(&self, _printer: &Printer) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::SolvePart2;

    use crate::core::Solver;

    #[test]
    fn it_solves_the_example() {
        let xmas = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let mut solver = SolvePart2::new(xmas, 127);
        assert_eq!(solver.solve(), Some(62));
    }
}
