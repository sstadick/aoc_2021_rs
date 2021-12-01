use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day1 {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day1 {
    fn main(&self) -> Result<(), DynError> {
        let reader = File::open(&self.input).map(BufReader::new)?;
        let mut depths = vec![];
        for line in reader.lines() {
            let line = line?;
            depths.push(line.parse::<usize>()?);
        }
        let mut depth_increases = 0;
        let mut prev_depth = None;
        for depth in depths {
            if let Some(prev) = prev_depth {
                if depth > prev {
                    depth_increases += 1;
                }
            }
            prev_depth = Some(depth)
        }
        println!("Total depth increases: {}", depth_increases);
        Ok(())
    }
}
