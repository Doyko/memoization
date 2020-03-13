#[cfg(test)]
mod tests {
    use crate::f;
    use crate::j;
    use crate::memoization;
    #[test]
    fn functions() {
        assert_eq!(f(1), 1);
        assert_eq!(f(2), 2);
        assert_eq!(j(1), false);
        assert_eq!(j(2), true);

        let mut g = memoization(f);
        let mut i = memoization(j);

        assert_eq!(g(1), 1);
        assert_eq!(g(2), 2);
        assert_eq!(g(1), 1);
        assert_eq!(i(1), false);
        assert_eq!(i(2), true);
        assert_eq!(i(1), false);
    }
}
