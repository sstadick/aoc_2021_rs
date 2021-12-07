use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use crate::utils::{slurp_file, ParseError};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day7b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day7b {
    fn main(&self) -> Result<(), DynError> {
        let positions: Positions = slurp_file(&self.input)?.remove(0);
        println!("Mean: {:?}", positions.mean_plus_minus());
        println!("Median: {}", positions.median());
        let (mean_low, mean_high) = positions.mean_plus_minus();
        println!("Choose lowest:");
        println!(
            "Cost mean low: {}",
            positions.converge_to(mean_low, |cost| (0..=cost).sum::<usize>())
        );
        println!(
            "Cost mean high: {}",
            positions.converge_to(mean_high, |cost| (0..=cost).sum::<usize>())
        );
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

    /// The mean itself is not the guaranteed minimum, but it is one of the terms
    /// when findind the point where the derivative of the cost function = 0. The other term
    /// ends up bounding our answer as mean +- 0.5.
    ///
    /// # References
    ///
    /// - https://www.reddit.com/r/adventofcode/comments/rawxad/2021_day_7_part_2_i_wrote_a_paper_on_todays/
    fn mean_plus_minus(&self) -> (usize, usize) {
        let mean = self.0.iter().sum::<usize>() as f64 / self.0.len() as f64;
        ((mean - 0.5).round() as usize, (mean + 0.5).round() as usize)
    }

    fn converge_to(&self, target: usize, cost: fn(usize) -> usize) -> usize {
        let mut total_cost = 0;
        for value in &self.0 {
            if *value >= target {
                total_cost += cost(*value - target);
            } else {
                total_cost += cost(target - *value);
            }
        }
        total_cost
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
