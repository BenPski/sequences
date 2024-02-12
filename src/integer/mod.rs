use self::{
    a000027::A000027,
    a001477::A001477,
    a001478::A001478,
    a001489::A001489,
    a000045::A000045,
    a000142::A000142};
use self::a005408::A005408;
use self::a005843::A005843;

pub mod a000027;
pub mod a000045;
pub mod a001477;
pub mod a001478;
pub mod a001489;
pub mod a000032;
pub mod a000204;
pub mod a000142;
pub mod a000004;
pub mod a000012;
pub mod a000035;
pub mod a000034;
pub mod a134451;
pub mod a040001;
pub mod a026741;
pub mod a005408;
pub mod a005843;

// if a sequence has a common name give it one
pub type Naturals = A000027;
pub type NonNegative = A001477;
pub type Negative = A001478;
pub type NonPositive = A001489;
pub type Fibonnaci = A000045;
pub type Factorial = A000142;
pub type Odd = A005408;
pub type Even = A005843;
