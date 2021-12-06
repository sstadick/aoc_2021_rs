use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use crate::utils::{slurp_file, ParseError};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day6a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day6a {
    fn main(&self) -> Result<(), DynError> {
        println!("EX: {:?}", self.input);
        let mut starter_fish: LanternFish = slurp_file(&self.input)?.remove(0);

        for _i in 0..80 {
            let mut new = vec![];
            for fish in &mut starter_fish.0 {
                if *fish > 0 {
                    *fish -= 1;
                } else {
                    *fish = 6;
                    new.push(8);
                }
            }
            starter_fish.0.extend(new.into_iter());
        }
        println!("Answer: {}", starter_fish.0.len());
        Ok(())
    }
}

/// The called bingo numbers
#[derive(Debug, Clone)]
struct LanternFish(Vec<usize>);

impl FromStr for LanternFish {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = vec![];
        for v in s.split(',') {
            values.push(v.parse::<usize>().map_err(|e| ParseError::new(e.to_string()))?);
        }
        Ok(LanternFish(values))
    }
}
