/*
 You are given wrong code and you need to find the mistake in it and fix it, in order for the sort to work properly.
 A Rust version was lacking, so I thought that I would add it myself.
 See if you can find the problem ;)
*/
use std::io;

fn insertionSort(n: usize, mut numbers: Vec<i32>) {
    let mut value = 0;

    for i in 1..n {
        value = numbers[i];
        let mut j = i-1;

        while j > 0 && value < numbers[j] {
            numbers[j+1] = numbers[j];
            j = j - 1;
        }
        numbers[j+1] = value;
    }

    for num in numbers {
        print!("{} ", num);
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut n_str = String::new();
    stdin.read_line(&mut n_str);
    let mut numbers_str = String::new();
    stdin.read_line(&mut numbers_str);

    let n: usize = n_str.trim().parse().unwrap();
    let mut numbers: Vec<i32> = numbers_str.trim().split_whitespace().map(|a| a.parse().unwrap()).collect();
    insertionSort(n, numbers);
}
