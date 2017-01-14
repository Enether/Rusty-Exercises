/*
Given five positive integers, find the minimum and maximum values that can be calculated by summing exactly four of the five integers. 
Then print the respective minimum and maximum values as a single line of two space-separated long integers.
*/
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = &mut String::new();
    stdin.read_line(input);

    let sums: (i64, i64) = get_numbers_sum(input);
    let (min_sum, max_sum) = sums;
    println!("{0} {1}", min_sum, max_sum);
}

fn get_numbers_sum(numbers: &mut String) -> (i64, i64) {
    // Return the minimum and maximum sum of the newly-created vector
    let vec: Vec<i64> = numbers.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let sum = vec.iter().fold(0, |a, &b| a+b);

    (sum - vec.iter().max().unwrap(), sum - vec.iter().min().unwrap())
}