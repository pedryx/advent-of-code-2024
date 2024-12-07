use std::time::Instant;
use itertools::Itertools;
use rayon::prelude::*;

type Num = u64;

fn create_exp(mut n: Num) -> Num {
    let mut pow = 1;
    while n > 0 {
        pow *= 10;
        n /= 10;
    }
    
    pow
}

fn is_true(test_value: Num, numbers: &[Num], allow_concatenation: bool, current_value: Num, index: usize) -> bool {
    let Some(&next_number) = numbers.get(index) else {
        return test_value == current_value;
    };

    if current_value > test_value {
        return false;
    }

    is_true(test_value, numbers, allow_concatenation, current_value + next_number, index + 1)
        || is_true(test_value, numbers, allow_concatenation, current_value * next_number, index + 1)
        || (allow_concatenation && {
            let concatenation = current_value * create_exp(next_number) + next_number;
            is_true(test_value, numbers, allow_concatenation, concatenation, index + 1)
        })
}

fn solve() -> (Num, Num) {
    let (result1, result2) = include_str!("../in.txt")
        .lines()
        .par_bridge()
        .map(|line|{
            let (test_value, numbers) = line.split_once(':').unwrap();
            let test_value = test_value.parse().unwrap();
            let numbers = numbers.split_ascii_whitespace().map(|number| number.parse().unwrap()).collect_vec();

            let part1 = if is_true(test_value, &numbers, false, numbers[0], 1) { test_value } else { 0 };
            let part2 = if is_true(test_value, &numbers, true, numbers[0], 1) { test_value } else { 0 };

            (part1, part2)
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    (result1, result2)
}

fn main() {
    let now = Instant::now();
    let (result1, result2) = solve();
    let elapsed = now.elapsed();

    println!("part1: {}", result1);
    println!("part2: {}", result2);
    println!("elapsed: {:.2?}", elapsed);
}
