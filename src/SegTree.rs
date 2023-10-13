pub struct SegTree {
    pub tree: Vec<usize>,
    pub Monoid: usize,
    pub eval: F,
}

impl<F: Fn(usize, usize) -> usize> SegmentTree<F> {
    fn from(m: usize, d: Vec<usize>, formula: F) -> SegTree<F> {
        let size = d.len().next_power_of_two();
        let mut v = vec![m; size * 2];
    }
}
