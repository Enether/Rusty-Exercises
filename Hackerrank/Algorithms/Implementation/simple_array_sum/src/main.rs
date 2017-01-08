use std::io;
use std::io::prelude::*;

fn main() {
    let mut stdin = io::stdin();
    let _junk_ = &mut String::new();
    let input = &mut String::new();
    stdin.read_line(_junk_);
    stdin.read_line(input);
    println!("{}",get_numbers_sum(input));
}

fn get_numbers_sum(numbers: &str) -> i32 {
    let vec: Vec<i32> = numbers.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let sum = vec.iter().fold(0, |a, &b| a+b);
    return sum;
}