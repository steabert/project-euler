// Special Pythagorean triplet

// A Pythagorean triplet is a set of three natural numbers,
// a < b < c, for which, a2 + b2 = c2

// For example, 32 + 42 = 9 + 16 = 25 = 52.

// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn main() {
    let mut answer: u32 = 0;
    let n_max: u32 = 1000;

    'outer: for a in 0..=n_max {
        for b in a + 1..=(n_max - a) {
            let c = n_max - a - b;
            if c > b && a * a + b * b == c * c {
                println!("{}, {}, {}", a, b, c);
                answer = a * b * c;
                break 'outer;
            }
        }
    }

    println!("answer = {}", answer);
}
