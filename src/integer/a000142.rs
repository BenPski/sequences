/// https://oeis.org/A000142
/// Factorials
///

use num::BigInt;

pub struct A000142 {
    n: BigInt,
    f: BigInt,
}

impl Default for A000142 {
    fn default() -> Self {
        A000142 { n: BigInt::from(0), f: BigInt::from(1) }
    }
}

impl Iterator for A000142 {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let orig = self.f.clone();
        self.n += 1;
        self.f = &self.n*&self.f;
        Some(orig)
    }
}
