pub mod day0;
pub mod day1;
pub mod day1b;

use std::error::Error;

use enum_dispatch::enum_dispatch;

pub type DynError = Box<dyn Error>;

#[enum_dispatch]
pub trait CommandImpl {
    fn main(&self) -> Result<(), DynError>;
}
