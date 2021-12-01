# aoc_2021_rs

This repo contains solutions for the [2021 Advent of Code](https://adventofcode.com/) in Rust.

## Running solutions

```bash
cargo run -- day0 --example test
```

## Adding a new day

`commands/day0.rs` is a template for all coming days create quick and easy subcommands for running solutions.

Copy the template to the new day:

```bash
cp src/commands/day0.rs src/commands/day?.rs
```

Make the new day public to the `src/commands/mod.rs`

Add the new day to the `SubCommand` struct in `src/main.rs`

Some days will have multiple parts, from experience you should create a new subcommand for the second part, EX day2b.rs, and modify from there instead of changing your answer for part one as some of the next days may rely on day1 part a.
