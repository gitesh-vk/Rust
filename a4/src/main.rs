fn is_prime(num: u64) -> bool {
    num > 1 && !(2..num).any(|i| num % i == 0)
}

fn main() {
    let num = 18;
    println!("{} is{} a prime number.", num, if is_prime(num) { "" } else { " not" });
}
