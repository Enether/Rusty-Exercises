fn is_square_integer(num: i32) -> bool{
    let square_int: f64 = (num as f64).sqrt();
    // see if the number is an integer
    let is_integer: bool = (square_int as i64 as f64) == square_int && square_int != 0.0;
    is_integer
}

fn main() {
    let stdin = std::io::stdin();
    let test_case_str = &mut String::new();
    stdin.read_line(test_case_str);

    for _ in 0..test_case_str.trim().parse().unwrap() {
        let start_end_str = &mut String::new();
        stdin.read_line(start_end_str);
        let start_end_values: Vec<i32> = start_end_str.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let start: i32 = start_end_values[0];
        let end: i32 = start_end_values[1];
        let mut count = 0;
        let mut first_sqrt = 0;
        let mut second_sqrt = 0;
        let mut num: i32 = start;
        while num <= end {
            if second_sqrt != 0 && first_sqrt != 0 {
                count += 1;
                let difference: i32 = second_sqrt - first_sqrt;
                
                num += (difference + 2);

                // update square roots
                let temp_sqrt: i32 = second_sqrt;
                first_sqrt = second_sqrt;
                second_sqrt = num;
                continue;
            }
            if is_square_integer(num) {
                if first_sqrt == 0 {
                    count += 1;
                    
                    first_sqrt = num;
                } else if second_sqrt == 0 {
                    second_sqrt = num;
                    continue;  // next iter wants to add from the number we're at
                }
            }
            num += 1;
        }

        println!("{:?}", count);
    }
}

#[cfg(test)]
mod tests {
    use is_square_integer;

    #[test]
    fn test_valid_square_integers() {
        assert_eq!(is_square_integer(4), true);
        assert_eq!(is_square_integer(9), true);
    }

    #[test]
    fn test_invalid_square_integers() {
        assert_eq!(is_square_integer(3), false);
        assert_eq!(is_square_integer(0), false);
    }
}