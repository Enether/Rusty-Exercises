//  https://www.hackerrank.com/challenges/sock-merchant
use std::io;
use std::collections::HashMap;


fn main() {
    let stdin = io::stdin();
    let _a = &mut String::new();
    stdin.read_line(_a);

    let socks_str = &mut String::new();
    stdin.read_line(socks_str);
    let socks: Vec<i32> = socks_str.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut colors: HashMap<i32, i32> = HashMap::new();
    for sock in socks {
        // update the color
        let color = colors.entry(sock).or_insert(0);
        *color += 1;
    }
    
    let result: i32 = colors.values().map(|a| a/2).fold(0, std::ops::Add::add);
    println!("{:?}", result);
}
