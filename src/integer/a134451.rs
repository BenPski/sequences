use std::{iter::Chain, vec::IntoIter};

use num::BigInt;

use super::a000034::A000034;

/// https://oeis.org/A134451
///

pub struct A134451 {
    iter: Chain<IntoIter<BigInt>, A000034>,
}

impl Default for A134451 {
    fn default() -> Self {
        Self { iter: vec![0.into()].into_iter().chain(A000034::default()) }
    }
}

impl Iterator for A134451 {  
    type Item = BigInt;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
