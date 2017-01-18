use std::io;

fn find_poisoned_id(prisoners: Vec<i64>) -> i64 {
    let prisoners_count: i64 = prisoners[0];
    let sweets_count: i64 = prisoners[1];
    let start_prisoner: i64 = prisoners[2];
    let mut poisoned: i64 = (start_prisoner + sweets_count - 1) % prisoners_count;
    if poisoned == 0 {
        poisoned = prisoners_count;
    }
    poisoned
}

fn main() {
    let stdin = io::stdin();
    let test_case_str = &mut String::new();
    stdin.read_line(test_case_str);
    let test_case_count: i32 = test_case_str.trim().parse().unwrap();

    for i in 0..test_case_count {
        let numbers_str = &mut String::new();
        stdin.read_line(numbers_str);
        let numbers: Vec<i64> = numbers_str.split_whitespace().map(|x| x.parse().unwrap()).collect();

        println!("{:?}", find_poisoned_id(numbers));
    }
}

#[cfg(test)]
mod tests {
    use find_poisoned_id;
    fn test_default_case() {
        assert!(find_poisoned_id(vec![5, 2, 1]) == 2);
    }
}
