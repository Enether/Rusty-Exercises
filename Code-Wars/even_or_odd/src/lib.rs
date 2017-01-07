pub mod even_or_odd;

#[cfg(test)]
mod tests {
    use even_or_odd::even_or_odd;
    #[test]
    fn it_works() {
    }
    #[test]
    fn returns_expected() {
        assert_eq!(even_or_odd(0), "Even");
        assert_eq!(even_or_odd(2), "Even");
        assert_eq!(even_or_odd(1), "Odd");
        assert_eq!(even_or_odd(7), "Odd");
    }
}
