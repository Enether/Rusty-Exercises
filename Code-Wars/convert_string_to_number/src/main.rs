/*
Note: This kata is inspired by Convert a Number to a String!. Try that one too.

Description

We need a function that can transform a string into a number. What ways of achieving this do you know?

Note: Don't worry, all inputs will be strings, and every string is a perfectly valid representation of an integral number.

Examples

```c++ string_to_number("1234") == 1234 string_to_number("605") == 605 string_to_number("1405") == 1405 string_to_number("-7") == -7
*/
fn main() {
    string_to_number("12");
}

fn string_to_number(s: &str) -> i32 {
    let result: i32 = s.parse().unwrap();
    result
}

#[test]
fn returns_expected() {
  assert_eq!(string_to_number("1234"), 1234);
  assert_eq!(string_to_number("605"), 605);
  assert_eq!(string_to_number("1405"), 1405);
  assert_eq!(string_to_number("-7"), -7);
}