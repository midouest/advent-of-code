use std::{cell::RefCell, rc::Rc};

use advent::day01::Day01;
use advent::day02::Day02;
use advent::{
    core::{
        controller::run,
        hub::{Hub, HubEvent},
        puzzle::{Puzzle, PuzzleRegistry},
    },
    day03::Day03,
};

const NUM_DAYS: usize = 26;

fn main() {
    let c = Rc::new(RefCell::new(cursive::default()));
    c.borrow_mut().set_fps(30);

    let puzzles: Vec<Box<dyn Puzzle>> = vec![
        Box::new(Day01::new()),
        Box::new(Day02::new()),
        Box::new(Day03::new()),
    ];
    let registry =
        PuzzleRegistry::with_puzzles(NUM_DAYS, puzzles).expect("Failed to register puzzles");

    let hub = Hub::new(registry);
    run::<Hub, HubEvent>(hub, Rc::clone(&c));
}
