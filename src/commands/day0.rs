use clap::Parser;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day0 {
    #[clap(long, short)]
    example: String,
}

impl CommandImpl for Day0 {
    fn main(&self) -> Result<(), DynError> {
        println!("EX: {}", self.example);
        Ok(())
    }
}
