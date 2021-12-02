pub mod day0;
pub mod day1;
pub mod day1b;
pub mod day2;
pub mod day2b;

use std::error::Error;

use enum_dispatch::enum_dispatch;

pub type DynError = Box<dyn Error + 'static>;

#[enum_dispatch]
pub trait CommandImpl {
    fn main(&self) -> Result<(), DynError>;
}