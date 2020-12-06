pub const NUM_ROWS: i64 = 128;
pub const NUM_COLS: i64 = 8;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Bound {
    low: i64,
    high: i64,
}

impl Bound {
    pub fn new(low: i64, high: i64) -> Self {
        Self { low, high }
    }

    pub fn split_low(&mut self) {
        self.high = (self.high - self.low) / 2 + self.low;
    }

    pub fn split_high(&mut self) {
        self.low = (self.high - self.low) / 2 + 1 + self.low;
    }
}

pub fn seat_id(row: i64, col: i64) -> i64 {
    row * NUM_COLS + col
}

pub struct Resolver {
    instructions: Vec<char>,
    i: usize,
    row_bound: Bound,
    col_bound: Bound,
}

impl Resolver {
    pub fn new(seat: &str) -> Self {
        let instructions = seat.chars().collect();
        Self {
            instructions,
            i: 0,
            row_bound: Bound::new(0, NUM_ROWS - 1),
            col_bound: Bound::new(0, NUM_COLS - 1),
        }
    }

    pub fn is_resolved(&self) -> bool {
        self.i == self.instructions.len()
    }

    pub fn seat_id(&self) -> i64 {
        seat_id(self.row_bound.low, self.col_bound.low)
    }

    pub fn resolve(&mut self) -> i64 {
        while !self.is_resolved() {
            self.step();
        }
        self.seat_id()
    }

    pub fn step(&mut self) {
        let c = self.instructions[self.i];

        match c {
            'F' => self.row_bound.split_low(),
            'B' => self.row_bound.split_high(),
            'L' => self.col_bound.split_low(),
            'R' => self.col_bound.split_high(),
            _ => (),
        };

        self.i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::Resolver;

    #[test]
    fn it_resolves_the_examples() {
        assert_eq!(Resolver::new("FBFBBFFRLR").resolve(), 357);
        assert_eq!(Resolver::new("BFFFBBFRRR").resolve(), 567);
        assert_eq!(Resolver::new("FFFBBBFRRR").resolve(), 119);
        assert_eq!(Resolver::new("BBFFBBFRLL").resolve(), 820);
    }
}
