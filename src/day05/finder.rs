use std::collections::HashSet;

use super::resolver::{seat_id, Resolver, NUM_COLS, NUM_ROWS};

pub struct SeatFinder {
    passes: Vec<String>,
    i: usize,
    seats: HashSet<i64>,
    found: Option<i64>,
}

impl SeatFinder {
    pub fn new(passes: Vec<String>) -> Self {
        let last_seat_id = seat_id(NUM_ROWS, NUM_COLS);
        let seats = (0..last_seat_id + 1).collect();
        Self {
            passes,
            i: 0,
            seats,
            found: None,
        }
    }

    pub fn is_found(&self) -> bool {
        self.found.is_some()
    }

    pub fn step(&mut self) {
        if self.i < self.passes.len() {
            self.resolve_next();
        } else {
            self.find_seat();
        }
    }

    fn resolve_next(&mut self) {
        let pass = &self.passes[self.i];
        let seat_id = Resolver::new(pass).resolve();
        self.seats.remove(&seat_id);
        self.i += 1;
    }

    fn find_seat(&mut self) {
        self.found = Some(
            *self
                .seats
                .iter()
                .find(|&&id| !self.seats.contains(&(id - 1)) && !self.seats.contains(&(id + 1)))
                .unwrap(),
        );
    }

    pub fn find(&mut self) -> i64 {
        while !self.is_found() {
            self.step();
        }
        self.found.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::parse_lines;

    use super::SeatFinder;

    #[test]
    fn test() {
        let passes: Vec<String> = parse_lines("input/day05/boarding_passes.txt").unwrap();
        let mut finder = SeatFinder::new(passes);
        assert_eq!(finder.find(), 0);
    }
}
