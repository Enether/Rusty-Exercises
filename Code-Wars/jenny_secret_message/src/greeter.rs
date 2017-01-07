pub fn greet(input : &str) -> String {
    if input == "Johnny" {
        return String::from("Hello, my love!");
    }

    return format!("Hello, {}!", input);
}