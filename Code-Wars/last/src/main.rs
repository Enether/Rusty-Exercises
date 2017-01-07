extern crate mylib;
use mylib::last::last;
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", last(&arr));
}
