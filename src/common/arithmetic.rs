use std::collections::VecDeque;

use num::BigInt;

/// Arithmetic sequences

pub trait Additive {
    fn step(&mut self);
}

pub struct Recurrent {
    coeffs: Vec<BigInt>,
    values: VecDeque<BigInt>,
    index: usize,
}

impl Default for Recurrent {
    fn default() -> Self {
        Recurrent { coeffs: vec![1.into(), 1.into()], values: VecDeque::from(vec![0.into(), 1.into()]), index: 0 }
    }
}

impl Iterator for Recurrent {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.values.len()-1 {
            let index = self.index;
            self.index += 1;
            Some(self.values[index].clone())
        } else {
            let orig = self.values[self.values.len()-1].clone();
            let mut val = 0.into();
            for i in 0..self.coeffs.len() {
                val += &self.coeffs[i]*&self.values[i];
            }
            self.values.pop_front();
            self.values.push_back(val);
            Some(orig)

        }
    }
}
