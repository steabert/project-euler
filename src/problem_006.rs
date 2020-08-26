// Sum square difference

// Find the difference between the sum of the squares of the
// first one hundred natural numbers and the square of the sum.

fn main() {
    let mut answer: u32 = 0;
    let n_max: u32 = 100;

    for n in 1..=n_max {
        answer += n.pow(2);
    }
    answer = ((n_max * (n_max + 1)) / 2).pow(2) - answer;

    println!("answer = {}", answer);
}
