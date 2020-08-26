// Consecutive prime sum

// The prime 41, can be written as the sum of six consecutive primes:
// 41 = 2 + 3 + 5 + 7 + 11 + 13

// This is the longest sum of consecutive primes that adds to a prime below one-hundred.

// The longest sum of consecutive primes below one-thousand that adds to a prime,
// contains 21 terms, and is equal to 953.

// Which prime, below one-million, can be written as the sum of the most consecutive primes?

fn main() {
    let mut answer = 0;
    let mut longest_run = 1;

    for p_start in 2..1_000_000 {
        if !is_prime(p_start) {
            continue;
        }
        let mut prime_sum = p_start;
        let mut run = 1;
        for p in p_start + 1..1_000_000 {
            if is_prime(p) {
                prime_sum += p;
                if prime_sum > 1_000_000 {
                    break;
                }
                run += 1;
                if is_prime(prime_sum) && run > longest_run {
                    longest_run = run;
                    answer = prime_sum;
                    println!("new run = {}, {}, {}", p_start, run, prime_sum);
                }
            }
        }
    }

    println!("answer = {}", answer);
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
