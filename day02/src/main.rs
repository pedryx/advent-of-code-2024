use std::time::Instant;
use itertools::Itertools;

fn is_safe(record: impl Iterator<Item = i32> + Clone) -> bool
{
    let diffs = record.tuple_windows().map(|(a, b)| b - a);
    (diffs.clone().all(|d| d > 0) || diffs.clone().all(|d| d < 0)) && diffs.clone().all(|d| d.abs() <= 3)
}

fn main() {
    let now = Instant::now();
    let records = include_str!("../in.txt")
        .lines()
        .map(|l| l.split_ascii_whitespace().map(|t| t.parse::<i32>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let result_part1 = records.iter()
        .filter(|r| is_safe(r.iter().copied()))
        .count();

    let result_part2 = records.iter()
        .filter(|record| (0..record.len())
            .any(|i| is_safe(record.iter().enumerate().filter(|(j, _)| i != *j).map(|(_, v)| *v)))
        ).count();
    let elapsed = now.elapsed();

    println!("part1: {:?}", result_part1);
    println!("part2: {:?}", result_part2);
    println!("elapsed: {:.2?}", elapsed);
}
