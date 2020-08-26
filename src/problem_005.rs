// Smallest multiple

// 2520 is the smallest number that can be divided by
// each of the numbers from 1 to 10 without any remainder.

// What is the smallest positive number that is evenly
// divisible by all of the numbers from 1 to 20?

fn main() {
    let mut answer: usize = 1;

    for p in 2..=20 {
        if is_prime(p) {
            let mut largest_exp = 0;
            for mut i in p..=20 {
                let mut exp = 0;
                while i % p == 0 {
                    exp += 1;
                    i /= p;
                }
                largest_exp = std::cmp::max(largest_exp, exp)
            }
            answer *= p.pow(largest_exp);
        }
    }

    println!("answer = {}", answer);
}

fn is_prime(n: usize) -> bool {
    if n == 2 {
        return true;
    }
    for i in 2..=((n as f64).sqrt().ceil() as usize) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
