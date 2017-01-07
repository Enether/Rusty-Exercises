/*x Simple, given a string of words, return the length of the shortest word(s).

String will never be empty and you do not need to account for different data types.*/
extern crate lib;
use lib::shortest_word::find_short;
fn main() {
    println!("{}", find_short("Dve e dva puti poveche").to_string());
}
