use self::{a000027::A000027, a001477::A001477, a001478::A001478, a001489::A001489, a000045::A000045, a000142::A000142};

pub mod a000027;
pub mod a000045;
pub mod a001477;
pub mod a001478;
pub mod a001489;
pub mod a000032;
pub mod a000204;
pub mod a000142;
pub mod a000004;

// if a sequence has a common name give it one
pub type Naturals = A000027;
pub type NonNegative = A001477;
pub type Negative = A001478;
pub type NonPositive = A001489;
pub type Fibonnaci = A000045;
pub type Factorial = A000142;
