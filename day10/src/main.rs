use std::time::Instant;
use glam::IVec2;
use itertools::Itertools;
use rustc_hash::FxHashSet;

static DIRS: &[IVec2] = &[IVec2::new(1, 0), IVec2::new(-1, 0), IVec2::new(0, -1), IVec2::new(0, 1)];

fn in_bounds(input: &[Vec<i8>], pos: IVec2) -> bool {
    (0..input[0].len() as i32).contains(&pos.x) && (0..input.len() as i32).contains(&pos.y)
}

fn process_position(input: &[Vec<i8>], trail_heads: &mut FxHashSet<IVec2>, pos: IVec2,) -> IVec2 {
    let current_height = input[pos.y as usize][pos.x as usize];
    if current_height == 9 {
        return IVec2::new(trail_heads.insert(pos) as i32, 1);
    }

    let mut sum = IVec2::ZERO;
    for dir in DIRS {
        let neighbor_pos = pos + dir;
        
        if !in_bounds(input, neighbor_pos) {
            continue;
        }
        if input[neighbor_pos.y as usize][neighbor_pos.x as usize] != current_height + 1 {
            continue;
        }

        sum += process_position(input, trail_heads, neighbor_pos);
    }

    sum
}

fn solve() -> (i32, i32) {
    let input = include_str!("../in.txt")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i8).collect_vec())
        .collect_vec();

    let mut sum = IVec2::ZERO;
    for (y, row) in input.iter().enumerate() {
        for (x, &height) in row.iter().enumerate() {
            if height != 0 {
                continue;
            }

            sum += process_position(&input, &mut FxHashSet::default(), IVec2::new(x as i32, y as i32));
        }
    }

    (sum.x, sum.y)
}

fn main() {
    let now = Instant::now();
    let (result1, result2) = solve();
    let elapsed = now.elapsed();

    println!("part1: {}", result1);
    println!("part2: {}", result2);
    println!("elapsed: {:.2?}", elapsed);
}
