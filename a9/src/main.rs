fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let original = "Test-String";
    let reversed = reverse_string(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}
