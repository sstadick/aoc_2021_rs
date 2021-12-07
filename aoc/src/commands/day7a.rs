use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use crate::utils::{slurp_file, ParseError};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day7a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day7a {
    fn main(&self) -> Result<(), DynError> {
        let positions: Positions = slurp_file(&self.input)?.remove(0);
        println!("Median: {}", positions.median());
        println!("Cost: {}", positions.converge_to_median());
        Ok(())
    }
}

/// The called bingo numbers
#[derive(Debug, Clone)]
struct Positions(Vec<usize>);

impl Positions {
    fn median(&self) -> usize {
        let mut copy = self.0.clone();
        copy.sort();
        let mid = copy.len() / 2;
        copy[mid]
    }

    fn converge_to_median(&self) -> usize {
        let median = self.median();
        let mut cost = 0;
        for value in &self.0 {
            if *value >= median {
                cost += *value - median;
            } else {
                cost += median - *value;
            }
        }
        cost
    }
}

impl FromStr for Positions {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = vec![];
        for v in s.split(',') {
            values.push(v.parse::<usize>().map_err(|e| ParseError::new(e.to_string()))?);
        }
        Ok(Self(values))
    }
}
