use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseIoError {
    #[error("Could not read file")]
    Io(#[from] io::Error),

    #[error("Could not parse file")]
    Parse,
}

pub type ParseIoResult<T> = Result<T, ParseIoError>;

/// Read each line of the file at the given path and convert it to type T
pub fn parse_lines<T: FromStr>(path: &str) -> ParseIoResult<Vec<T>> {
    use ParseIoError::*;

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line?.parse().map_err(|_| Parse))
        .collect()
}
