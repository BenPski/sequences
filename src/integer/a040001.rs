use std::{iter::Chain, vec::IntoIter};

use num::BigInt;

use super::a000034::A000034;

/// https://oeis.org/A040001
///

pub struct A040001 {
    iter: Chain<IntoIter<BigInt>, A000034>,
}

impl Default for A040001 {
    fn default() -> Self {
        Self { iter: vec![1.into()].into_iter().chain(A000034::default()) }
    }
}

impl Iterator for A040001 {  
    type Item = BigInt;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
