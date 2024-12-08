use std::time::Instant;
use glam::I16Vec2;
use itertools::Itertools;
use rustc_hash::FxHashMap;

type Num = i16;
type Pos = I16Vec2;

fn in_bounds(pos: Pos, size: Num) -> bool {
    (0..size).contains(&pos.x) && (0..size).contains(&pos.y)
}

fn process_direction_part1(antinode: Pos, size: Num, antinodes_map: &mut [bool], result: &mut usize) {
    if !in_bounds(antinode, size) {
        return;
    }

    let index = (antinode.y * size + antinode.x) as usize;
    if !antinodes_map[index] {
        antinodes_map[index] = true;
        *result += 1;
    }
}

fn process_direction_part2(start: Pos, direction: Pos, size: Num, antinodes_map: &mut [bool], result: &mut usize) {
    let mut current = start;
    while in_bounds(current, size) {
        let index = (current.y * size + current.x) as usize;
        if !antinodes_map[index] {
            antinodes_map[index] = true;
            *result += 1;
        }
        current += direction;
    }
}

fn solve() -> (usize, usize) {
    let input = include_str!("../in.txt");
    let size = input.lines().next().unwrap().len() as Num;

    let map = input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .enumerate()
        .filter(|&(_, c)| c != '.')
        .fold(FxHashMap::default(), |mut map: FxHashMap<char, Vec<_>>, (i, c)| {
            map.entry(c).or_default().push(Pos::new(i as Num % size, i as Num / size));
            map
        });
    let antennas = map
        .values()
        .flat_map(|antennas| antennas.iter().tuple_combinations());

    let mut antinodes_map_part1 = vec![false; (size * size) as usize];
    let mut antinodes_map_part2 = vec![false; (size * size) as usize];
    let mut result1 = 0;
    let mut result2 = 0;

    for (&antenna1, &antenna2) in antennas {
        let direction = antenna2 - antenna1;

        process_direction_part1(antenna2 + direction, size, &mut antinodes_map_part1, &mut result1);
        process_direction_part1(antenna1 - direction, size, &mut antinodes_map_part1, &mut result1);

        process_direction_part2(antenna2, direction, size, &mut antinodes_map_part2, &mut result2);
        process_direction_part2(antenna1, -direction, size, &mut antinodes_map_part2, &mut result2);
    }

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
