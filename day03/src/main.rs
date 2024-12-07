use std::time::Instant;
use regex::Regex;

fn main() {
    let now = Instant::now();
    let result_part1 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap()
        .captures_iter(include_str!("../in.txt"))
        .map(|capture| capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap())
        .sum::<i32>();

    let mut enabled = true;
    let mut result_part2 = 0;
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap()
        .captures_iter(include_str!("../in.txt"))
        .for_each(|capture| match &capture[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => if enabled { result_part2 += capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap() },
        });
    let elapsed = now.elapsed();

    println!("part1: {:?}", result_part1);
    println!("part2: {:?}", result_part2);
    println!("elapsed: {:.2?}", elapsed);
}