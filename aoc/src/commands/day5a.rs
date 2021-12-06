#![allow(dead_code)]
use std::{collections::HashMap, path::PathBuf, str::FromStr};

use clap::Parser;

use crate::utils::{slurp_file, ParseError};

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day5a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day5a {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<Line> = slurp_file(&self.input)?;
        let mut seen_points = HashMap::new();

        for line in lines.iter().filter(|l| l.is_horizontal() || l.is_vertical()) {
            println!("{:?}", line);
        }

        for line in &lines {
            if line.is_horizontal() {
                let mut x = line.start.x;
                let y = line.start.y;
                while x <= line.stop.x {
                    let point = Point::new(x, y);
                    let p = seen_points
                        .entry(point.as_cantor_pairing() as i64)
                        .or_insert_with(|| Point::new(x, y));
                    p.counter += 1;
                    x += 1;
                }
            } else if line.is_vertical() {
                let x = line.start.x;
                let mut y = line.start.y;
                while y <= line.stop.y {
                    let point = Point::new(x, y);
                    let p = seen_points
                        .entry(point.as_cantor_pairing() as i64)
                        .or_insert_with(|| Point::new(x, y));
                    p.counter += 1;
                    y += 1;
                }
            }
        }
        // count the number of keys that have counter of two
        let mut total = 0;
        for (_k, v) in seen_points.into_iter() {
            if v.counter >= 2 {
                total += 1;
            }
        }
        println!("Answer: {}", total);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    counter: usize,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y, counter: 0 }
    }

    fn as_cantor_pairing(&self) -> f64 {
        (((self.x + self.y) * (self.x + self.y + 1)) as f64 / 2.0) + self.y as f64
    }

    fn from_cantor_pairing(pairing: f64) -> Self {
        let w = (((8.0 * pairing + 1.0).sqrt() - 1.0) / 2.0).floor();
        let t = (w.powi(2) + w) / 2.0;
        let y = pairing - t;
        let x = w - y;
        Self::new(x as i64, y as i64)
    }
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(',');
        let x = values
            .next()
            .expect("Missing x")
            .parse::<i64>()
            .map_err(|e| ParseError::new(e.to_string()))?;
        let y = values
            .next()
            .expect("Missing y")
            .parse::<i64>()
            .map_err(|e| ParseError::new(e.to_string()))?;
        Ok(Point::new(x, y))
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    stop: Point,
    c_start: f64,
    c_stop: f64,
}

impl Line {
    fn new(start: Point, stop: Point) -> Self {
        let c_start = start.as_cantor_pairing();
        let c_stop = stop.as_cantor_pairing();
        // Force "sorted" order
        if c_start < c_stop {
            Self { start, stop, c_start, c_stop }
        } else {
            Self { start: stop, stop: start, c_start: c_stop, c_stop: c_start }
        }
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.stop.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.stop.y
    }

    fn slope(&self) -> f64 {
        (self.stop.y - self.start.y) as f64 / (self.stop.x - self.start.x) as f64
    }

    // fn walk_points(&self) {
    //     // y = mx + b
    //     let mut x = self.start.x;
    //     let mut y = self.start.y;

    // }
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split(" -> ");
        let start = points
            .next()
            .expect("Missing Line Start")
            .parse::<Point>()
            .map_err(|e| ParseError::new(e.to_string()))?;
        let stop = points
            .next()
            .expect("Missing Line Stop")
            .parse::<Point>()
            .map_err(|e| ParseError::new(e.to_string()))?;
        Ok(Self::new(start, stop))
    }
}
