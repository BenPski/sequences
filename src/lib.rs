use std::{collections::BinaryHeap, ops::Mul};

use num::BigInt;
pub mod primes;
pub mod integer;
pub mod common;


#[derive(Debug, Clone)]
pub struct Factorial {
    n: BigInt,
    f: BigInt,
}

impl Factorial {
    pub fn new() -> Self {
        Factorial { n: BigInt::from(0), f: BigInt::from(1) }
    }
}

impl Iterator for Factorial {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let orig = self.f.clone();
        self.n += 1;
        self.f = &self.n*&self.f;
        Some(orig)
    }
}

