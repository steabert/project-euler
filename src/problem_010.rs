// Summation of primes.

// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

// Find the sum of all the primes below two million.

fn main() {
    let mut prime_sum: usize = 0;

    for p in 2..2_000_000 {
        if is_prime(p) {
            prime_sum += p as usize;
        }
    }

    println!("answer = {}", prime_sum);
}

fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    }
    for i in 2..=((n as f64).sqrt().ceil() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
