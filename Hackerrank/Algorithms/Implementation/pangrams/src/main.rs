use std::io;
use std::collections::HashSet;

fn main() {
    let mut stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line);
    line = line.to_lowercase();
    let mut characters = HashSet::new();
    for character in line.chars() {
        if character.is_alphabetic() {
            characters.insert(character);
        }
    }

    if characters.len() == 26 {
        println!("pangram");
    } else {
        println!("not pangram");
    }
}
