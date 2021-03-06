pub mod commands;
pub mod utils;

use commands::*;
use enum_dispatch::enum_dispatch;

use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[enum_dispatch(CommandImpl)]
#[derive(Parser, Debug)]
enum SubCommand {
    Day0(day0::Day0),
    Day1(day1::Day1),
    Day1b(day1b::Day1b),
    Day2(day2::Day2),
    Day2b(day2b::Day2b),
    Day3a(day3a::Day3a),
    Day3b(day3b::Day3b),
    Day4a(day4a::Day4a),
    Day4b(day4b::Day4b),
    Day5a(day5a::Day5a),
    Day5b(day5b::Day5b),
    Day6a(day6a::Day6a),
    Day6b(day6b::Day6b),
}
fn main() -> Result<(), DynError> {
    let opts = Opts::parse();

    opts.subcommand.main()
}
