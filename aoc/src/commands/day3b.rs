use std::{error::Error, fmt, path::PathBuf, str::FromStr};

use clap::Parser;

use crate::utils::slurp_file;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day3b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day3b {
    fn main(&self) -> Result<(), DynError> {
        let diagnostics: Vec<DiagnosticReportLine> = slurp_file(&self.input)?;

        let co2_mask = u64::MAX << diagnostics[0].width;
        let mut oxygen: Option<u64> = None;
        let mut co2: Option<u64> = None;
        let mut mask = !(u64::MAX >> 1);

        let mut oxygen_possible_values = diagnostics.clone();
        let mut co2_possible_values = diagnostics.clone();

        for _ in 0..u64::BITS {
            if oxygen.is_none() {
                let most_common = count_zeros_and_ones(&oxygen_possible_values, mask);
                oxygen_possible_values = oxygen_possible_values
                    .into_iter()
                    .filter(|DiagnosticReportLine { value, width: _ }| {
                        most_common.is_most_common(mask & value)
                    })
                    .collect();
                if oxygen_possible_values.len() == 1 {
                    oxygen = Some(oxygen_possible_values[0].value);
                }
            }

            if co2.is_none() {
                let most_common = count_zeros_and_ones(&co2_possible_values, mask);
                co2_possible_values = co2_possible_values
                    .into_iter()
                    .filter(|DiagnosticReportLine { value, width: _ }| {
                        // Skip over all values that fall outside the width
                        ((co2_mask & mask) > 0) || most_common.is_least_common(mask & value)
                    })
                    .collect();
                if co2_possible_values.len() == 1 {
                    co2 = Some(co2_possible_values[0].value);
                }
            }
            if oxygen.is_some() && co2.is_some() {
                break;
            }

            mask = mask.rotate_right(1);
        }

        println!("Oxygen: {}", oxygen.unwrap());
        println!("CO2: {}", co2.unwrap());
        println!("Answer: {}", oxygen.unwrap() * co2.unwrap());

        Ok(())
    }
}

#[derive(Debug)]
pub enum MostCommon {
    One,
    Zero,
}

impl MostCommon {
    fn is_most_common(&self, value: u64) -> bool {
        match self {
            MostCommon::One => value > 0,
            MostCommon::Zero => value == 0,
        }
    }

    fn is_least_common(&self, value: u64) -> bool {
        match self {
            MostCommon::One => value == 0,
            MostCommon::Zero => value > 0,
        }
    }
}

fn count_zeros_and_ones(diagnostics: &[DiagnosticReportLine], mask: u64) -> MostCommon {
    let mut ones = 0;
    let mut zeros = 0;
    for DiagnosticReportLine { value, width: _ } in diagnostics {
        if (value & mask) > 0 {
            ones += 1;
        } else {
            zeros += 1;
        }
    }

    if ones >= zeros {
        MostCommon::One
    } else {
        MostCommon::Zero
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
