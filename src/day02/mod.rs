pub mod part1;
pub mod part2;

use cursive::{
    theme::{ColorStyle, Style},
    utils::span::SpannedStr,
    utils::span::SpannedString,
    Cursive, Printer,
};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::{cell::RefCell, collections::HashSet, marker::PhantomData, rc::Rc, str::FromStr};
use thiserror::Error;

use crate::core::{
    fs::parse_lines,
    puzzle::{Puzzle, PuzzlePart},
    solver::Solver,
    solver::SolverController,
    util::last_n,
};

pub trait PasswordPolicy {
    fn validate(entry: &PasswordEntry) -> ValidatedPassword;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PasswordEntry {
    a: usize,
    b: usize,
    c: char,
    password: String,
}

impl PasswordEntry {
    pub fn new(a: usize, b: usize, c: char, password: String) -> Self {
        Self { a, b, c, password }
    }

    pub fn is_valid<T: PasswordPolicy>(&self) -> ValidatedPassword {
        T::validate(self)
    }
}

#[derive(Debug, Error)]
pub enum ParsePasswordEntryError {
    #[error("The password entry did not match the regex")]
    NoMatch,

    #[error("Missing capture group {0}")]
    MissingCapture(usize),

    #[error("Could not parse a component of the password entry")]
    CouldNotParse(usize),
}

impl FromStr for PasswordEntry {
    type Err = ParsePasswordEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParsePasswordEntryError::*;

        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
        }

        let captures: Captures = RE.captures(s).ok_or(NoMatch)?;
        // First capture is the entire string

        let a: usize = captures
            .get(1)
            .ok_or(MissingCapture(1))?
            .as_str()
            .parse()
            .map_err(|_| CouldNotParse(1))?;

        let b: usize = captures
            .get(2)
            .ok_or(MissingCapture(2))?
            .as_str()
            .parse()
            .map_err(|_| CouldNotParse(2))?;

        let c: char = captures
            .get(3)
            .ok_or(MissingCapture(3))?
            .as_str()
            .chars()
            .next()
            .ok_or(CouldNotParse(3))?;

        let password = captures.get(4).ok_or(MissingCapture(4))?.as_str();

        Ok(PasswordEntry::new(a, b, c, password.to_string()))
    }
}

pub struct OldPolicy {}

impl PasswordPolicy for OldPolicy {
    fn validate(entry: &PasswordEntry) -> ValidatedPassword {
        let min = entry.a;
        let max = entry.b;

        let mut valid = HashSet::new();
        let mut invalid = HashSet::new();

        let mut count = 0;
        for (i, c) in entry.password.chars().enumerate() {
            if c != entry.c {
                continue;
            }

            count += 1;
            if count > max {
                invalid.insert(i);
                break;
            } else {
                valid.insert(i);
            }
        }

        let is_a_valid = count >= min;
        let is_b_valid = invalid.len() == 0;

        ValidatedPassword::new(entry.clone(), is_a_valid, is_b_valid, valid, invalid)
    }
}

pub struct NewPolicy {}

impl PasswordPolicy for NewPolicy {
    fn validate(entry: &PasswordEntry) -> ValidatedPassword {
        let mut valid = HashSet::new();
        let mut invalid = HashSet::new();
        let mut is_a_valid = false;
        let mut is_b_valid = false;

        match (
            entry.password.chars().nth(entry.a - 1),
            entry.password.chars().nth(entry.b - 1),
        ) {
            (Some(c1), Some(c2)) => {
                if (c1 == entry.c && c2 != entry.c) || (c1 != entry.c && c2 == entry.c) {
                    valid.insert(entry.a - 1);
                    valid.insert(entry.b - 1);
                    is_a_valid = true;
                    is_b_valid = true;
                } else {
                    invalid.insert(entry.a - 1);
                    invalid.insert(entry.b - 1);
                }
            }
            (c1, c2) => {
                is_a_valid = c1.is_some();
                is_b_valid = c2.is_some();
            }
        };

        ValidatedPassword::new(entry.clone(), is_a_valid, is_b_valid, valid, invalid)
    }
}

pub struct ValidatedPassword {
    entry: PasswordEntry,
    is_a_valid: bool,
    is_b_valid: bool,
    valid: HashSet<usize>,
    invalid: HashSet<usize>,
}

impl ValidatedPassword {
    pub fn new(
        entry: PasswordEntry,
        is_a_valid: bool,
        is_b_valid: bool,
        valid: HashSet<usize>,
        invalid: HashSet<usize>,
    ) -> Self {
        Self {
            entry,
            is_a_valid,
            is_b_valid,
            valid,
            invalid,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.is_a_valid && self.is_b_valid
    }

    pub fn to_styled(&self) -> SpannedString<Style> {
        let mut styled = SpannedString::new();

        let a = SpannedString::styled(self.entry.a.to_string(), self.get_style(self.is_a_valid));
        styled.append(a);
        styled.append("-");
        let b = SpannedString::styled(self.entry.b.to_string(), self.get_style(self.is_b_valid));
        styled.append(b);
        styled.append(" ");
        styled.append(self.entry.c);
        styled.append(": ");

        for (i, c) in self.entry.password.chars().enumerate() {
            if self.valid.contains(&i) {
                styled.append(SpannedString::styled(c, self.get_style(true)));
            } else if self.invalid.contains(&i) {
                styled.append(SpannedString::styled(c, self.get_style(false)));
            } else {
                styled.append(c);
            }
        }

        styled.append(" ... ");

        let valid_text = if self.is_valid() { "Valid" } else { "Invalid " };
        styled.append(SpannedString::styled(
            valid_text,
            self.get_style(self.is_valid()),
        ));

        styled
    }

    fn get_style(&self, valid: bool) -> Style {
        if valid {
            Style::from(ColorStyle::secondary())
        } else {
            Style::from(ColorStyle::tertiary())
        }
    }
}

pub struct SolveDay02<P: PasswordPolicy> {
    passwords: Vec<PasswordEntry>,
    i: usize,
    valid_count: i64,
    history: Vec<ValidatedPassword>,
    _phantom: PhantomData<P>,
}

impl<P> SolveDay02<P>
where
    P: PasswordPolicy,
{
    pub fn new(passwords: Vec<PasswordEntry>) -> Self {
        Self {
            passwords,
            i: 0,
            valid_count: 0,
            history: Vec::new(),
            _phantom: PhantomData,
        }
    }
}

impl<P> Solver for SolveDay02<P>
where
    P: PasswordPolicy,
{
    fn is_done(&self) -> bool {
        self.i == self.passwords.len()
    }

    fn solution(&self) -> Option<i64> {
        self.with_done_some(self.valid_count)
    }

    fn step(&mut self) {
        if self.is_done() {
            return;
        }

        let entry = &self.passwords[self.i];
        let validated = entry.is_valid::<P>();
        if validated.is_valid() {
            self.valid_count += 1;
        }
        self.history.push(validated);

        self.i += 1;
    }

    fn draw(&self, printer: &Printer) {
        let range = last_n(self.history.len(), printer.size.y);
        for (i, validated) in (&self.history[range]).iter().enumerate() {
            let styled = validated.to_styled();
            printer.print_styled((0, i), SpannedStr::from(&styled));
        }
    }
}

#[derive(Debug)]
pub struct Day02 {}

impl Day02 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Puzzle for Day02 {
    fn get_title(&self) -> String {
        "Password Philosophy".to_string()
    }

    fn is_implemented(&self, _part: PuzzlePart) -> bool {
        true
    }

    fn run(&self, part: PuzzlePart, c: Rc<RefCell<Cursive>>) {
        let passwords: Vec<PasswordEntry> =
            parse_lines("input/day02/password_database.txt").expect("Could not load puzzle input");
        if part == PuzzlePart::One {
            let solver = SolveDay02::<OldPolicy>::new(passwords);
            let controller = SolverController::new(solver);
            controller.run(c);
        } else {
            let solver = SolveDay02::<NewPolicy>::new(passwords);
            let controller = SolverController::new(solver);
            controller.run(c);
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::{NewPolicy, OldPolicy, PasswordEntry};

//     #[test]
//     fn it_validates_old_passwords() {
//         assert_eq!(
//             PasswordEntry::new(1, 3, 'a', "abcde".to_string()).is_valid::<OldPolicy>(),
//             true
//         );

//         assert_eq!(
//             PasswordEntry::new(1, 3, 'b', "cdefg".to_string()).is_valid::<OldPolicy>(),
//             false
//         );

//         assert_eq!(
//             PasswordEntry::new(2, 9, 'c', "cccccccc".to_string()).is_valid::<OldPolicy>(),
//             true
//         );
//     }

//     #[test]
//     fn it_validates_new_passwords() {
//         assert_eq!(
//             PasswordEntry::new(1, 3, 'a', "abcde".to_string()).is_valid::<NewPolicy>(),
//             true
//         );

//         assert_eq!(
//             PasswordEntry::new(1, 3, 'b', "cdefg".to_string()).is_valid::<NewPolicy>(),
//             false
//         );

//         assert_eq!(
//             PasswordEntry::new(2, 9, 'c', "cccccccc".to_string()).is_valid::<NewPolicy>(),
//             false
//         );
//     }

//     #[test]
//     fn it_parses_from_str() {
//         let s = "1-3 a: abcde";
//         let entry = s.parse::<PasswordEntry>().unwrap();
//         assert_eq!(entry, PasswordEntry::new(1, 3, 'a', "abcde".to_string()));
//     }
// }
