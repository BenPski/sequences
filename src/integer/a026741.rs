use num::{BigInt, Integer};

/// https://oeis.org/A026741
///

pub struct A026741 {
    n: BigInt,
}

impl Default for A026741 {
    fn default() -> Self {
        // offset to make stepping not require a clone
        Self { n: (-1).into() }
    }
}

impl Iterator for A026741 {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;
        if self.n.is_even() {
            Some(self.n.clone()/2)
        } else {
            Some(self.n.clone())
        }
    }
}
