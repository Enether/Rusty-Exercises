use std::io;


fn print_vector(numbers: &Vec<i32>) {
    for num in numbers {
        print!("{} ", num);
    }
    println!("");
}


fn main() {
    let stdin = std::io::stdin();
    let mut count_str = String::new();
    stdin.read_line(&mut count_str);
    let mut numbers_str = String::new();
    stdin.read_line(&mut numbers_str);
    let mut numbers: Vec<i32> = numbers_str.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut index: usize = numbers.len() - 1;
    let mut origNumber: i32 = numbers[index];
    while true {
        if (index != 0 && numbers[index-1] > origNumber) {
            numbers[index] = numbers[index-1];
        } else {
            numbers[index] = origNumber;
            break;
        }
        index -= 1;
        print_vector(&numbers);
    }
    print_vector(&numbers);
    
}
