use std::io;

fn main() {
    let stdin = io::stdin();
    let mut n_k = &mut String::new();
    stdin.read_line(n_k);
    let n_k_vector: Vec<i32> = n_k.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let k: &i32 = n_k_vector.last().unwrap();

    let mut numbers_str = &mut String::new();
    stdin.read_line(numbers_str);
    let numbers: Vec<i32> = numbers_str.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut pairs_count: i32 = 0;
    let numbers_len: i32 = numbers.len() as i32;
    let mut idx: i32 = 0;
    
    /*
    Get all pairs by going through each number and from that number iterate till the end
    */
    while idx < numbers_len {
        let value: i32 = numbers[idx as usize];

        let mut idx_2: i32 = idx + 1;
        while idx_2 < numbers.len() as i32 {
            if (value + numbers[idx_2 as usize]) % k == 0 {
                pairs_count += 1
            } 
            idx_2 += 1
        }

        idx += 1;
    }
    println!("{}", pairs_count);
}
