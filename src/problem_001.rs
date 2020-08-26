
// Multiples of 3 and 5

// If we list all the natural numbers below 10 that are multiples
// of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

fn main() {
    let n = 1000;

    let mut sum = 0;
    for a in 3..n {
        if a % 3 == 0 {
            sum += a;
            continue;
        }
        if a % 5 == 0 {
            sum += a;
            continue;
        }
    }
    println!("total: {}", sum);
}
