use std::collections::HashSet;

use cursive::Printer;

use crate::core::{solver::Solver, util::last_n};

#[derive(Debug, Clone)]
pub struct Solve2Sum {
    elements: Vec<i64>,
    set: HashSet<i64>,
    sum: i64,
    i: usize,
    x: Option<i64>,
    y: Option<i64>,
    history: Vec<(i64, i64)>,
    found: Option<(i64, i64)>,
}

impl Solve2Sum {
    pub fn new(elements: Vec<i64>, sum: i64) -> Self {
        let set = elements.iter().cloned().collect();
        Self {
            elements,
            set,
            sum,
            i: 0,
            x: None,
            y: None,
            history: Vec::new(),
            found: None,
        }
    }
}

impl Solver<i64> for Solve2Sum {
    fn is_done(&self) -> bool {
        self.i >= self.elements.len() || self.found.is_some()
    }

    fn solution(&self) -> Option<i64> {
        self.found.map(|(x, y)| x * y)
    }

    fn step(&mut self) {
        if self.is_done() {
            return;
        }

        let x = self.elements[self.i];
        let y = self.sum - x;

        self.x = Some(x);
        self.y = Some(y);

        if self.set.contains(&y) {
            self.found = Some((x, y));
        } else {
            self.history.push((x, y));
            self.i += 1;
        }
    }

    fn draw(&self, printer: &Printer) {
        let range = last_n(self.history.len(), printer.size.y);
        let mut i = 0;
        for (x, y) in &self.history[range] {
            let line = format_line(*x, *y, x + y, false);
            printer.print((0, i), &line);
            i += 1;
        }

        if let Some((x, y)) = self.found {
            let line = format_line(x, y, self.sum, true);
            printer.print((0, i), &line);
        }
    }
}

fn format_line(x: i64, y: i64, sum: i64, found: bool) -> String {
    let found_text = if found { "Found!" } else { "Not found" };
    format!("Input {1}: {0} - {1} = {2} ... {3}", sum, x, y, found_text)
}
