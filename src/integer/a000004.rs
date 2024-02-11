pub struct A000004 {
}

impl Default for A000004 {
    fn default() -> Self {
        A000004 {  }
    }
}

impl Iterator for A000004 {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}
