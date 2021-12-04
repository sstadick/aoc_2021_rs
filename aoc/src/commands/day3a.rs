use std::{error::Error, fmt, path::PathBuf, str::FromStr};

use clap::Parser;

use crate::utils::slurp_file;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day3a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day3a {
    fn main(&self) -> Result<(), DynError> {
        let diagnostics: Vec<DiagnosticReportLine> = slurp_file(&self.input)?;

        let epsilon_mask = !(u64::MAX << diagnostics[0].width);
        let mut gamma: u64 = 0;
        let mut epsilon: u64 = 0;
        let mut mask = !(u64::MAX >> 1);

        for _ in 0..u64::BITS {
            let mut zeros = 0;
            let mut ones = 0;
            for DiagnosticReportLine { value, width: _ } in &diagnostics {
                if (value & mask) > 0 {
                    ones += 1;
                } else {
                    zeros += 1;
                }
            }

            if ones > zeros {
                gamma |= mask;
            } else {
                epsilon |= mask;
            }

            mask = mask.rotate_right(1);
        }

        println!("Gamma: {}", gamma);
        println!("Epislon: {}", epsilon & epsilon_mask);
        println!("Answer: {}", gamma * (epsilon & epsilon_mask));

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticError {
    msg: String,
}

impl Error for DiagnosticError {}

impl fmt::Display for DiagnosticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error for command: {}", self.msg)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct DiagnosticReportLine {
    value: u64,
    width: u64,
}

impl FromStr for DiagnosticReportLine {
    type Err = DiagnosticError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.len() as u64;
        let value =
            u64::from_str_radix(s, 2).map_err(|e| DiagnosticError { msg: e.to_string() })?;
        Ok(DiagnosticReportLine { value, width })
    }
}
