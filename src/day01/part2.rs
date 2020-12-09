use std::collections::HashSet;

use cursive::Printer;

use crate::core::{solver::Solver, util::last_n};

#[derive(Debug, Clone)]
pub struct Solve3Sum {
    elements: Vec<i64>,
    set: HashSet<i64>,
    sum: i64,
    i: usize,
    found: Option<(i64, i64, i64)>,
}

impl Solve3Sum {
    pub fn new(elements: Vec<i64>, sum: i64) -> Self {
        let set = elements.iter().cloned().collect();
        Self {
            elements,
            set,
            sum,
            i: 0,
            found: None,
        }
    }
}

impl Solver<i64> for Solve3Sum {
    fn is_done(&self) -> bool {
        self.i >= self.elements.len() || self.found.is_some()
    }

    fn solution(&self) -> Option<i64> {
        self.found.map(|(x, y, z)| x * y * z)
    }

    fn step(&mut self) {
        if self.is_done() {
            return;
        }

        let x = self.elements[self.i];
        for j in 0..self.elements.len() {
            let y = self.elements[j];
            let z = self.sum - x - y;
            let xyz = (x, y, z);

            if self.set.contains(&z) {
                self.found = Some(xyz);
                return;
            }
        }

        self.i += 1;
    }

    fn draw(&self, printer: &Printer) {
        let mut i = 0;
        let range = last_n(self.i, printer.size.y - 1);
        for input in &self.elements[range] {
            let line = format!("Input {} ... Not found", input);
            printer.print((0, i), &line);
            i += 1;
        }

        if let Some((x, y, z)) = self.found {
            let line = format_line(x, y, z, self.sum);
            printer.print((0, i), &line);
        }
    }
}

fn format_line(x: i64, y: i64, z: i64, sum: i64) -> String {
    format!("Input {0}: {3} - {0} - {1} = {2} ... Found!", x, y, z, sum)
}
