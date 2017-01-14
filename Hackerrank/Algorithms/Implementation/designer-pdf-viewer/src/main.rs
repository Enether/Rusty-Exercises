use std::io;

fn main() {
    let stdin = io::stdin();
    let mut heights = &mut String::new();
    let mut word = &mut String::new();
    stdin.read_line(heights);
    stdin.read_line(word);

    let parsed_heights: Vec<i32> = heights.split_whitespace().map(|a| a.parse().unwrap()).collect();
    let highlighted_area = get_max_letter_height(word, parsed_heights) * word.len() as i32;
    println!("{}", highlighted_area);
}

fn get_max_letter_height(word: &mut String, heights: Vec<i32>) -> i32 {
    let start_letter: u32 = 'a' as u32;
    let mut max: i32 = 0;
    for letter in word.chars() {
        let current_value: i32 = heights[(letter as u32 - start_letter) as usize];
        if current_value > max {
            max = current_value;
        }
    }
    max
}