use cursive::Cursive;
use std::{cell::RefCell, fmt, rc::Rc};
use thiserror::Error;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PuzzlePart {
    One,
    Two,
}

pub trait Puzzle: fmt::Debug {
    fn get_title(&self) -> String;
    fn is_implemented(&self, part: PuzzlePart) -> bool;
    fn run(&self, part: PuzzlePart, c: Rc<RefCell<Cursive>>);
}

#[derive(Debug, Error)]
pub enum PuzzleRegistryError {
    #[error("Puzzle day must be between 1 and {1}, got {0}")]
    DayOutOfBounds(usize, usize),

    #[error("Another puzzle is already registered for day {0}")]
    AlreadyRegistered(usize),
}

pub type Result<T> = std::result::Result<T, PuzzleRegistryError>;

#[derive(Debug)]
pub struct PuzzleRegistry {
    num_days: usize,
    puzzles: Vec<Option<Box<dyn Puzzle>>>,
}

impl PuzzleRegistry {
    pub fn new(num_days: usize) -> Self {
        let mut puzzles = Vec::with_capacity(num_days);
        (0..num_days).for_each(|_| puzzles.push(None));

        Self { num_days, puzzles }
    }

    pub fn with_puzzles(num_days: usize, puzzles: Vec<Box<dyn Puzzle>>) -> Result<Self> {
        let mut registry = Self::new(num_days);
        registry.register(puzzles)?;
        Ok(registry)
    }

    pub fn register(&mut self, puzzles: Vec<Box<dyn Puzzle>>) -> Result<()> {
        puzzles
            .into_iter()
            .enumerate()
            .map(|(i, puzzle)| self.insert(i, puzzle))
            .collect()
    }

    pub fn has(&self, day: usize) -> bool {
        day < self.num_days && self.puzzles[day].is_some()
    }

    pub fn insert(&mut self, day: usize, puzzle: Box<dyn Puzzle>) -> Result<()> {
        if day >= self.num_days {
            return Err(PuzzleRegistryError::DayOutOfBounds(day, self.num_days));
        }

        if let Some(_) = &self.puzzles[day] {
            return Err(PuzzleRegistryError::AlreadyRegistered(day));
        }

        self.puzzles[day] = Some(puzzle);

        Ok(())
    }

    pub fn get(&self, day: usize) -> &Option<Box<dyn Puzzle>> {
        if day >= self.num_days {
            return &None;
        }

        &self.puzzles[day]
    }

    pub fn get_puzzles(&self) -> &Vec<Option<Box<dyn Puzzle>>> {
        &self.puzzles
    }
}
