use std::io;

fn main() {
    let mut stdin = io::stdin();
    let mut house_start_end = String::new();
    stdin.read_line(&mut house_start_end);
    let s_t: Vec<i64> = house_start_end.split_whitespace().map(|a| a.parse::<i64>().unwrap()).collect();
    let house_start: i64 = s_t[0];
    let house_end: i64 = s_t[1];

    let mut tree_coords = String::new();
    stdin.read_line(&mut tree_coords);
    let a_b: Vec<i64> = tree_coords.split_whitespace().map(|a| a.parse::<i64>().unwrap()).collect();
    let apple_tree: i64 = a_b[0];
    let orange_tree: i64 = a_b[1];

    // useless
    stdin.read_line(&mut house_start_end);

    let mut apples_str = String::new();
    stdin.read_line(&mut apples_str);
    let apples: Vec<i64> = apples_str.split_whitespace().map(|a| a.parse::<i64>().unwrap() + apple_tree).collect();
    
    let mut oranges_str = String::new();
    stdin.read_line(&mut oranges_str);
    let oranges: Vec<i64> = oranges_str.split_whitespace().map(|a| a.parse::<i64>().unwrap() + orange_tree).collect();
    
    let mut orange_count = 0;
    for orange in oranges {
        if orange >= house_start && orange <= house_end {
            orange_count += 1;
        }
    }
    let mut apple_count = 0;
    for apple in apples {
        if apple >= house_start && apple <= house_end {
            apple_count += 1;
        }
    }
    println!("{}", apple_count);
    println!("{}", orange_count);

}