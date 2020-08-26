// Largest palyndrome product

// A palindromic number reads the same both ways.
// The largest palindrome made from the product of
// two 2-digit numbers is 9009 = 91 × 99.

// Find the largest palindrome made from the product of
// two 3-digit numbers.

fn main() {
    let mut answer: usize = 0;

    for a in 0..1000 {
        'b: for b in 0..1000 {
            let candidate = (a * b).to_string().into_bytes();
            let size = candidate.len();
            for i in 0..size / 2 {
                if candidate[i] != candidate[size - 1 - i] {
                    continue 'b;
                }
            }
            answer = std::cmp::max(answer, a * b);
        }
    }

    println!("answer = {}", answer);
}
