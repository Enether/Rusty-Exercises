pub fn greet(input : &str) -> String {
    match input {
        "Johnny" => String::from("Hello, my love!"),
        _ => format!("Hello, {}!", input),  
    }
}