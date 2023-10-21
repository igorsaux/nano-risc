pub fn align_to_mult(n: usize, m: usize) -> usize {
    (n + (m - 1)) & !(m - 1)
}
