use std::io;

fn decide_kangaroos_result<'a>(kangaroos: Vec<i32>) -> &'a str {
    let first_kangaroo: i32 = kangaroos[0];
    let first_jump: i32 = kangaroos[1];

    let second_kangaroo: i32 = kangaroos[2];
    let second_jump: i32 = kangaroos[3];

    let original_distance: i32 = second_kangaroo - first_kangaroo;
    let rate_of_catching_up: i32 = first_jump - second_jump;
    if rate_of_catching_up <= 0 || original_distance % rate_of_catching_up != 0 {
        "NO"
    } else {
        "YES"
    }
}

fn main() {
    let stdin = io::stdin();
    let mut kangaroos_str = &mut String::new();
    stdin.read_line(kangaroos_str);

    let kangaroos: Vec<i32> = kangaroos_str.split_whitespace().map(|k| k.parse().unwrap()).collect();
    let result = decide_kangaroos_result(kangaroos);
    println!("{}", result);
}