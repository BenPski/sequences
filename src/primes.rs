use std::{collections::BinaryHeap, ops::Mul};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wheel {
    value: usize,
    wheel: Vec<usize>,
    index: usize,
}

impl Wheel {
    fn new(value: usize, wheel: Vec<usize>) -> Self {
        Wheel { value, wheel, index: 0 }
    }

    fn shift(&self) -> Self {
        Wheel { value: self.value + self.wheel[self.index], wheel: self.wheel.clone(), index: (self.index + 1) % self.wheel.len() }
    }

    fn next(&mut self) -> usize {
        let orig = self.value;
        self.value += self.wheel[self.index];
        self.index = (self.index + 1) % self.wheel.len();
        orig
    }
    
}

fn make_wheel(n: usize) -> (Vec<usize>, Vec<usize>) {
    if n == 0 {
        (vec![2], vec![1])
    } else {
        let (prev_primes, prev_cycle) = make_wheel(n-1);
        let mut steps = prev_cycle.into_iter().cycle();
        let p = *prev_primes.clone().last().unwrap();
        let mut last = p + steps.next().unwrap();
        let mut primes = prev_primes.clone();
        primes.push(last);
        let (mut diffs, mut tot) = (Vec::new(), prev_primes.into_iter().product::<usize>());
        while tot > 0 {
            let mut n = last + steps.next().unwrap();
            while n % p <= 0 {
                n += steps.next().unwrap();
            }
            let d = n - last;
            diffs.push(d);
            tot -= d;
            last = n;
        }
        (primes, diffs)
    }
}

impl Mul<usize> for Wheel {
    type Output = Wheel;

    fn mul(self, rhs: usize) -> Self::Output {
        Wheel { value: self.value*rhs, wheel: self.wheel.into_iter().map(|x| x*rhs).collect(), index: self.index }
    }
}

impl Ord for Wheel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for Wheel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Primes {
    candidates: Wheel,
    composites: BinaryHeap<Wheel>,
    initial: Vec<usize>,
}

impl Default for Primes {
    fn default() -> Self {
        Self::new()
    }
}

impl Primes {
    pub fn new() -> Self {
        // 3 was a decent balance in testing
        let (mut primes, vals) = make_wheel(3);
        primes.reverse();
        let wheel = Wheel::new(primes[0], vals);
        Primes { candidates: wheel, composites: BinaryHeap::new(), initial: primes[1..].to_vec() }
    }

    fn step(&mut self) -> usize {
        if let Some(n) = self.initial.pop() {
            return n;
        }
        loop {
            let n = self.candidates.value;
            if let Some(comp) = self.composites.peek() {
                if comp.value == n {
                    self.step_composites(n);
                    self.candidates.next();
                } else {
                    self.step_composites(n);
                    self.composites.push(self.candidates.clone()*n);
                    self.candidates.next();
                    return n;
                }
            } else {
                self.composites.push(self.candidates.clone()*n);
                self.candidates.next();
                return n;
            }
        } 
    }

    fn step_composites(&mut self, n: usize) {
        loop {
            if let Some(mut comp) = self.composites.peek_mut() {
                if comp.value <= n {
                    *comp = comp.shift();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}

impl Iterator for Primes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.step())
    }
}

#[test]
fn first_few() {
    let seq = Primes::default();
    let vals = seq.take(10).collect::<Vec<_>>();
    let known = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    assert_eq!(vals, known);
}
