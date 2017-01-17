use std::io;

fn main() {
    let stdin = io::stdin();
    let test_count_str = &mut String::new();
    stdin.read_line(test_count_str);
    let test_count: i32 = test_count_str.trim().parse().unwrap();
    for _ in 0..test_count {
        let cycles_str = &mut String::new();
        stdin.read_line(cycles_str);
        
        let cycles: i32 = cycles_str.trim().parse().unwrap();
        let mut height: i32 = 1;
        for j in 1..(cycles + 1) {
            if j % 2 != 0 {
                height *= 2;
            } else {
                height += 1;
            }
        }
        println!("{}", height);
    }
}