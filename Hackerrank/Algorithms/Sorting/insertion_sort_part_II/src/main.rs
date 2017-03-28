use std::io; 
 
 
fn print_vector(numbers: &Vec<i32>) { 
    for num in numbers { 
        print!("{} ", num); 
    } 
    println!(""); 
} 
 
fn num_is_sorted(numbers: &Vec<i32>, index: usize) -> bool {
    if (index == 0) {
        return true
    } else {
        return numbers[index-1] <= numbers[index];
    }
}
 
fn main() { 
    let stdin = std::io::stdin(); 
    let mut count_str = String::new(); 
    stdin.read_line(&mut count_str); 
    let mut numbers_str = String::new(); 
    stdin.read_line(&mut numbers_str); 
    let mut numbers: Vec<i32> = numbers_str.split_whitespace().map(|x| x.parse().unwrap()).collect(); 
 
    for num in 1..numbers.len() {
        let mut idx = num;  // reassign so that it can be mutable
        if !num_is_sorted(&numbers, idx) {
            while (!num_is_sorted(&numbers, idx)) {
                numbers.swap(idx, idx-1);
                idx -= 1;
            }
        }
        
        print_vector(&numbers)
    }
} 