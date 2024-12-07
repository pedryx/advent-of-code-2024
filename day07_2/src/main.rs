/*
 * Inspired by:
 * - https://github.com/jiribenes    | > Use checks and rev operations instead of naive forward recursion.
 * 
 * - https://github.com/maneatingape | > Better concat_check and create_exp functions.
 *                                   | > Part 2 can be skipped for current line if part 1 is success.
 */

use std::time::Instant;

use itertools::Itertools;
use rayon::prelude::*;

type Num = u64;
type Ops = [(fn(Num, Num) -> bool, fn(Num, Num) -> Num)];

const PART1_OPS: &Ops = &[(mul_check, mul_rev), (add_check, add_rev)];
const PART2_OPS: &Ops = &[(concat_check, concat_rev), (mul_check, mul_rev), (add_check, add_rev)];

fn create_exp(n: Num) -> Num {
    if n < 10 {
        10
    }
    else if n < 100 {
        100
    }
    else {
        1000
    }
}

fn concat_check(number: Num, target: Num) -> bool {
    target % create_exp(number) == number
}

fn concat_rev(number: Num, target: Num) -> Num {
    target / create_exp(number)
}

fn mul_check(number: Num, target: Num) -> bool {
    target % number == 0
}

fn mul_rev(number: Num, target: Num) -> Num {
    target / number
}

fn add_check(number: Num, target: Num) -> bool {
    number >= target
}

fn add_rev(number: Num, target: Num) -> Num {
    target - number
}

fn is_true(numbers: &[Num], ops: &Ops, current_value: Num, index: usize) -> bool {
    if index == 0 {
        return current_value == numbers[index];
    }
    
    for (check, rev_op) in ops {
        if !check(numbers[index], current_value) {
            continue;
        }

        let number = rev_op(numbers[index], current_value);
        if is_true(numbers, ops, number, index - 1) {
            return true;
        }
    }

    false
}

fn solve() -> (Num, Num) {
    let (result1, result2) = include_str!("../in_test.txt")
        .lines()
        .par_bridge()
        .map(|line|{
            let (test_value, numbers) = line.split_once(':').unwrap();
            let test_value = test_value.parse().unwrap();

            let numbers = numbers.split_ascii_whitespace().map(|number| number.parse().unwrap()).collect_vec();

            if is_true(&numbers, PART1_OPS, test_value, numbers.len() - 1) {
                (test_value, test_value)
            }
            else if is_true(&numbers, PART2_OPS, test_value, numbers.len() - 1) {
                (0, test_value)
            }
            else {
                (0, 0)
            }
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    (result1, result2)
}

fn main() {
    let now = Instant::now();
    let (part1, part2) = solve();
    let elapsed = now.elapsed();

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    println!("duration: {:.2?}", elapsed);
}
