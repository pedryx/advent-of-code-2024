use std::time::Instant;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rustc_hash::FxHashSet;

const OBSTACLE_TILE: char = '#';
const GUARD_TILE: char = '^';

type Num = i16;
type Vec2 = (Num, Num);
type Map = Vec<Vec<bool>>;

fn parse_map() -> (Map, Vec2) {
    let input = include_str!("../in.txt").lines();
    let mut guard_pos = Vec2::default();
    let mut map = Vec::new();

    for (y, line) in input.enumerate() {
        map.push(vec![false; line.len()]);
        for (x, c) in line.chars().enumerate() {
            match c {
                OBSTACLE_TILE => map[y][x] = true,
                GUARD_TILE => guard_pos = (x as Num, y as Num),
                _ => (),
            };
        }
    }

    (map, guard_pos)
}

fn in_bounds(map: &Map, pos: &Vec2) -> bool {
    0 <= pos.0 && pos.0 < map[0].len() as Num && 0 <= pos.1 && pos.1 < map.len() as Num
}

fn get_route(map: &Map, guard_pos: &Vec2, obstacle: Option<&Vec2>) -> (Vec<Vec2>, bool) {
    let mut guard_pos = *guard_pos;
    let mut guard_dir = (0, -1);
    let mut is_loop = true;
    let mut visited = FxHashSet::default();

    while visited.insert((guard_pos, guard_dir)) {
        let next_pos = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);

        if !in_bounds(map, &next_pos) {
            is_loop = false;
            break;
        }

        if map[next_pos.1 as usize][next_pos.0 as usize] || obstacle.map_or(false, |&pos| next_pos == pos) {
            guard_dir = (-guard_dir.1, guard_dir.0);
        }
        else {
            guard_pos = next_pos;
        }
    }

    (visited.into_iter().map(|(pos, _)| pos).unique().collect(), is_loop)
}

fn solve() -> (Num, Num) {
    let (map, guard_pos) = parse_map();
    let (route, _) = get_route(&map, &guard_pos, None);

    let result1 = route.len() as Num;
    let result2 = route.into_par_iter()
        .filter(|&pos| pos != guard_pos)
        .filter(|pos| get_route(&map, &guard_pos, Some(pos)).1)
        .count() as Num;

    (result1, result2)
}

fn main() {
    let now = Instant::now();
    let (result1, result2) = solve();
    let elapsed = now.elapsed();

    println!("part1: {}", result1);
    println!("part2: {}", result2);
    println!("Elapsed time: {:.2?}", elapsed);
}