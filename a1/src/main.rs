fn is_palindrome(s: &str) -> bool {
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

fn main() {
    let test_string = "racecar";
    if is_palindrome(test_string) {
        println!("{} is a palindrome.", test_string);
    } else {
        println!("{} is not a palindrome.", test_string);
    }
}