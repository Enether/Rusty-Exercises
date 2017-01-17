//  https://www.hackerrank.com/challenges/strange-advertising
use std::io;

fn main() {
    let stdin = io::stdin();
    let days_str = &mut String::new();
    stdin.read_line(days_str);

    let days: i32 = days_str.trim().parse().unwrap();
    let mut people: i32 = 0;
    let mut new_people: i32 = 5;
    for i in 0..days {
        let people_liked: i32 = new_people / 2;
        new_people = people_liked * 3;
        people += people_liked;
    }

    println!("{:?}", people);
}