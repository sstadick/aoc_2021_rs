use std::{error::Error, fmt, path::PathBuf, str::FromStr};

use clap::Parser;

use super::{CommandImpl, DynError};
use crate::utils::slurp_file;

#[derive(Parser, Debug)]
pub struct Day2b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day2b {
    fn main(&self) -> Result<(), DynError> {
        let cmds: Vec<Cmd> = slurp_file(&self.input)?;
        let mut pos = Position::default();
        cmds.iter().for_each(|cmd| pos.update(cmd));
        eprintln!("{:?}", pos.x * pos.y);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub aim: i64,
}

impl Position {
    fn update(&mut self, cmd: &Cmd) {
        match cmd {
            Cmd::Forward(mag) => {
                self.x += mag;
                self.y += self.aim * mag;
            }
            Cmd::Down(mag) => self.aim += mag,
            Cmd::Up(mag) => self.aim -= mag,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), aim: Default::default() }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cmd {
    Forward(i64),
    Down(i64),
    Up(i64),
}

#[derive(Debug, Clone)]
pub struct CmdError {
    msg: String,
}

impl Error for CmdError {}

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error for command: {}", self.msg)
    }
}

impl FromStr for Cmd {
    type Err = CmdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let cmd_part = parts
            .next()
            .ok_or(CmdError { msg: String::from("Missing Command") })?
            .to_ascii_lowercase();
        let mag_part = parts
            .next()
            .ok_or(CmdError { msg: String::from("Missing Command Magnitude") })?
            .parse::<i64>()
            .map_err(|e| CmdError { msg: e.to_string() })?;

        match cmd_part.as_ref() {
            "forward" => Ok(Cmd::Forward(mag_part)),
            "down" => Ok(Cmd::Down(mag_part)),
            "up" => Ok(Cmd::Up(mag_part)),
            _ => Err(CmdError { msg: "Invalid command".into() }),
        }
    }
}
