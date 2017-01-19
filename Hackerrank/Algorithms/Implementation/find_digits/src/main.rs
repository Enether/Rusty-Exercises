use std::io;

fn find_digits_dividable_by_n(n: i32) -> i32 {
    let mut digits_count: i32 = 0;
    let mut number: i32 = n;
    
    while number > 0 {
        let digit: i32 = number % 10;
        if digit != 0 {
            if n % digit == 0 {
                digits_count += 1;
            }
        }
        
        number /= 10;
    }

    digits_count
}

fn main() {
    let stdin = io::stdin();
    let test_cases_count_str = &mut String::new();
    stdin.read_line(test_cases_count_str);
    let test_cases_count: i32 = test_cases_count_str.trim().parse().unwrap();

    for i in 0..test_cases_count {
        let number_str = &mut String::new();
        stdin.read_line(number_str);
        let number: i32 = number_str.trim().parse().unwrap();

        println!("{}", find_digits_dividable_by_n(number));
    }
}


#[cfg(test)]
mod tests {
    use find_digits_dividable_by_n;

    #[test]
    fn test_default_values() {
        assert!(find_digits_dividable_by_n(12) == 2);
        assert!(find_digits_dividable_by_n(1012) == 3);
        assert!(find_digits_dividable_by_n(36) == 2);
        assert!(find_digits_dividable_by_n(362) == 1);
    }
}