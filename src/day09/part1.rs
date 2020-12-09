use std::collections::HashSet;

use cursive::Printer;

use crate::core::Solver;

pub struct SolvePart1 {
    xmas: Vec<usize>,
    preamble: usize,
    i: usize,
    found: Option<usize>,
}

impl SolvePart1 {
    pub fn new(xmas: Vec<usize>, preamble: usize) -> Self {
        Self {
            i: preamble,
            xmas,
            preamble,
            found: None,
        }
    }
}

impl Solver<usize> for SolvePart1 {
    fn is_done(&self) -> bool {
        self.found.is_some() || self.i >= self.xmas.len()
    }

    fn solution(&self) -> Option<usize> {
        self.found
    }

    fn step(&mut self) {
        let x = self.xmas[self.i];

        let mut candidates = HashSet::new();

        let end = self.i;
        let begin = end - self.preamble;

        let mut found_sum = false;
        for j in begin..end {
            let y = self.xmas[j];
            if candidates.contains(&y) {
                found_sum = true;
                break;
            }

            if y > x {
                continue;
            }

            let z = x - y;
            candidates.insert(z);
        }

        if !found_sum {
            self.found = Some(x);
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
    fn it_solves_the_example() {
        let xmas = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let mut solver = SolvePart1::new(xmas, 5);
        assert_eq!(solver.solve(), Some(127));
    }
}
