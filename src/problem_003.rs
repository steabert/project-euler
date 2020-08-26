// Largest prime factor

// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

fn main() {
    let mut answer: usize = 0;

    let mut n: usize = 600_851_475_143;

    let mut i = 2;
    while n > 1 {
        if is_prime(i) {
            while n % i == 0 {
                answer = i;
                n = n / i;
            }
        }
        i += 1;
    }

    println!("answer = {}", answer);
}

fn is_prime(n: usize) -> bool {
    for i in 2..((n as f64).sqrt().ceil() as usize) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
