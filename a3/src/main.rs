fn shortest_word(string: &str) -> Option<&str> {
    string.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let input_string = "The quick brown fox jumps over the lazy dog";
    if let Some(shortest) = shortest_word(input_string) {
        println!("The shortest word in the string is: {}", shortest);
    } else {
        println!("The string is empty.");
    }
}
