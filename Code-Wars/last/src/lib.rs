pub mod last;

#[cfg(test)]
mod tests {
    use last::last;
    #[test]
    fn it_works() {
    }
    #[test]
    fn should_work_for_non_empty_list_of_integers() {
        assert_eq!(last(&[1, 2, 3]), 3);
    }

    #[test]
    fn should_work_for_non_empty_list_of_strings() {
        assert_eq!(last(&["a", "b"]), "b");
    }
}
