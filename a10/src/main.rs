fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    !(2..n).any(|i| n % i == 0)
}

fn main() {
    let number = 18;
    if is_prime(number) {
        println!("{} is a prime number.", number);
    } else {
        println!("{} is not a prime number.", number);
    }
}
