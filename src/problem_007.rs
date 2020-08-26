// 10001st prime

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
// we can see that the 6th prime is 13.

// What is the 10 001st prime number?

fn main() {
    let p_pos_max: u32 = 10001;

    let mut p = 2;
    let mut p_pos = 1;
    while p_pos < p_pos_max {
        p += 1;
        if is_prime(p) {
            p_pos += 1
        }
    }

    println!("answer = {}", p);
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
