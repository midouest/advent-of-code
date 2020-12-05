use std::ops::Range;

pub fn last_n(n: usize, max: usize) -> Range<usize> {
    if n <= max {
        0..n
    } else {
        n - max..n
    }
}
