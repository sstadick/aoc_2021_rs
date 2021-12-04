use std::{
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::Parser;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day4a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day4a {
    fn main(&self) -> Result<(), DynError> {
        let (numbers, mut boards) = read_file(&self.input)?;

        'bingo: for number in numbers.0 {
            for board in boards.iter_mut() {
                board.set_value(number);
                if board.has_won() {
                    let score = board.score_board(number);
                    println!("{}", board);
                    println!("Winner: {}", score);
                    break 'bingo;
                }
            }
        }

        Ok(())
    }
}

fn read_file(path: impl AsRef<Path>) -> Result<(Numbers, Vec<BingoBoard>), DynError> {
    let reader = File::open(path).map(BufReader::new)?;
    let mut lines = reader.lines();

    let numbers = lines.next().expect("Missing first line of file")?.parse::<Numbers>()?;
    let mut boards = vec![];

    let mut temp = vec![];
    for line in lines {
        let line = line?;
        if line.is_empty() {
            if !temp.is_empty() {
                boards.push(temp.join("\n").parse::<BingoBoard>()?);
            }
            temp.clear();
            continue;
        }
        temp.push(line);
    }
    if !temp.is_empty() {
        boards.push(temp.join("\n").parse::<BingoBoard>()?);
    }
    Ok((numbers, boards))
}

// TODO: make a simple macro for making these error types
#[derive(Debug, Clone)]
struct ParseError {
    msg: String,
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error for command: {}", self.msg)
    }
}

/// The called bingo numbers
#[derive(Debug, Clone)]
struct Numbers(Vec<usize>);

impl FromStr for Numbers {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = vec![];
        for v in s.split(',') {
            values.push(v.parse::<usize>().map_err(|e| ParseError { msg: e.to_string() })?);
        }
        Ok(Numbers(values))
    }
}

#[derive(Debug, Clone)]
struct BCell {
    value: usize,
    marked: bool,
}

impl BCell {
    fn new(value: usize) -> Self {
        BCell { value, marked: false }
    }
    fn mark_seen(&mut self) {
        self.marked = true
    }
}

#[derive(Debug, Clone)]
struct BingoBoard {
    rows: Vec<Vec<BCell>>,
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

impl BingoBoard {
    fn has_won(&self) -> bool {
        // Check row-wise win con
        for row in &self.rows {
            if row.iter().all(|cell| cell.marked) {
                return true;
            }
        }

        // Check col-wise win con
        for i in 0..self.rows[0].len() {
            if self.rows.iter().all(|r| r[i].marked) {
                return true;
            }
        }

        false
    }

    fn set_value(&mut self, value: usize) {
        // TODO: index the boards for easier checking and setting
        for row in self.rows.iter_mut() {
            for v in row.iter_mut() {
                if v.value == value {
                    v.mark_seen();
                }
            }
        }
    }

    fn score_board(&self, last_called_value: usize) -> usize {
        // Sum all unmarked
        let mut sum = 0;
        for row in &self.rows {
            for v in row {
                if !v.marked {
                    sum += v.value;
                }
            }
        }

        // Mult by last_called_value
        sum * last_called_value
    }
}

impl FromStr for BingoBoard {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = vec![];
        for raw_row in s.split('\n') {
            let mut row = vec![];
            for raw_value in raw_row.split_ascii_whitespace() {
                row.push(BCell::new(
                    raw_value.parse::<usize>().map_err(|e| ParseError { msg: e.to_string() })?,
                ));
            }
            rows.push(row);
        }
        Ok(BingoBoard { rows })
    }
}
