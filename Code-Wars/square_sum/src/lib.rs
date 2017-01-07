pub mod square_sum;
#[cfg(test)]
mod tests {
    use square_sum::square_sum;
    #[test]
    fn returns_expected() {
    assert_eq!(square_sum(vec![1, 2]), 5);
    assert_eq!(square_sum(vec![-1, -2]), 5);
    assert_eq!(square_sum(vec![5, 3, 4]), 50);
    }
    #[test]
    fn it_works() {
    }
}
