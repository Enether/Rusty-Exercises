pub fn find_short(s: &str) -> u32 {
    let mut shortest_length = u32::max_value();
    for word in s.split(" ") {
        let word_len: u32 = word.len() as u32;
        if word_len < shortest_length {
            shortest_length = word_len
        }
    }
    shortest_length
}