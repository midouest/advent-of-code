use crate::core::Grid;

pub struct SeatingModel {
    seats: Grid<char>,
    stable: bool,
}

impl SeatingModel {
    pub fn new(seats: Grid<char>) -> Self {
        Self {
            seats,
            stable: false,
        }
    }

    pub fn is_stable(&self) -> bool {
        self.stable
    }

    pub fn seats(&self) -> &Grid<char> {
        &self.seats
    }

    pub fn step(&mut self) {
        let mut next = self.seats.clone();

        let mut changed = false;
        for y in 0..self.seats.height() {
            for x in 0..self.seats.width() {
                let &prev_seat = self.seats.get(x, y).unwrap();
                if prev_seat == '.' {
                    continue;
                }

                let num_occupied = self.num_occupied_neighbors(x, y);
                let seat = if prev_seat == 'L' && num_occupied == 0 {
                    '#'
                } else if prev_seat == '#' && num_occupied >= 4 {
                    'L'
                } else {
                    prev_seat
                };

                if seat != prev_seat {
                    next.set(x, y, seat);
                    changed = true;
                }
            }
        }

        if !changed {
            self.stable = true;
        }

        self.seats = next;
    }

    pub fn simulate(&mut self) -> &Grid<char> {
        while !self.is_stable() {
            self.step();
        }
        self.seats()
    }

    fn num_occupied_neighbors(&self, x: usize, y: usize) -> usize {
        self.seats
            .neighbors(x, y)
            .unwrap()
            .into_iter()
            .filter(|&&c| c == '#')
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::SeatingModel;

    const EXAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    const FINAL: &str = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";

    #[test]
    fn it_models_the_example() {
        let seats = EXAMPLE.parse().unwrap();
        let expected = FINAL.parse().unwrap();
        let mut model = SeatingModel::new(seats);
        assert_eq!(model.simulate(), &expected);
    }
}
