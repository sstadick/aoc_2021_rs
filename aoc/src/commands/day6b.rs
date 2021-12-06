use std::{collections::VecDeque, path::PathBuf, str::FromStr};

use clap::Parser;

use crate::utils::{slurp_file, ParseError};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day6b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day6b {
    fn main(&self) -> Result<(), DynError> {
        let starter_fish: LanternFish = slurp_file(&self.input)?.remove(0);

        let mut lookup = VecDeque::new();
        for i in 0..=6 {
            lookup.push_back(FishCounter { generations: i, total_fish: 0 });
        }

        for fish in starter_fish.0 {
            lookup[fish].total_fish += 1;
        }

        let mut hatchery = VecDeque::new();
        for i in 7..=8 {
            hatchery.push_back(FishCounter { generations: i, total_fish: 0 });
        }

        for _i in 0..256 {
            // Add new fish
            let new_fish_to_add = lookup[0].total_fish;
            let young_fish_to_promote = hatchery[0].total_fish;

            lookup.rotate_left(1);
            for (i, group) in lookup.iter_mut().enumerate() {
                group.generations = i;
            }
            lookup.back_mut().unwrap().total_fish += young_fish_to_promote;

            hatchery.rotate_left(1);
            for (i, group) in hatchery.iter_mut().enumerate() {
                group.generations = i;
            }
            hatchery.back_mut().unwrap().total_fish = new_fish_to_add;
        }

        let answer = lookup.iter().chain(hatchery.iter()).map(|g| g.total_fish).sum::<usize>();
        println!("Answer: {}", answer);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
struct FishCounter {
    generations: usize,
    total_fish: usize,
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
