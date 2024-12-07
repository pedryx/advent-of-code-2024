use std::{collections::HashMap, time::Instant};
use itertools::Itertools;

type Num = u32;

fn solve_part1((a, b): (&[Num], &[Num])) -> Num {
    a.iter()
        .sorted()
        .zip(b.iter().sorted())
        .map(|(&x, &y)| x.abs_diff(y))
        .sum()
}

fn solve_part2((a, b): (&[Num], &[Num])) -> Num {
    let counter = b.iter()
        .fold(HashMap::new(), |mut map, &v| { *map.entry(v).or_insert(0) += 1; map });

    a.iter()
        .map(|v| v * counter.get(v).unwrap_or(&0))
        .sum()
}

fn main() {
    let now = Instant::now();
    let input: (Vec<_>, Vec<_>) = include_str!("../in1.txt")
        .split_ascii_whitespace()
        .map(|n| n.parse::<Num>().unwrap())
        .tuples()
        .unzip();
    let elapsed = now.elapsed();

    println!("part1: {:?}", solve_part1((&input.0, &input.1)));
    println!("part2: {:?}", solve_part2((&input.0, &input.1)));
    println!("elapsed: {:.2?}", elapsed);
}
