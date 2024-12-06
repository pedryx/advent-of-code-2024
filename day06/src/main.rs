use std::{collections::HashSet, time::Instant};
use glam::I16Vec2;
use itertools::iproduct;
use rayon::prelude::*;

const EMPTY_TILE: char = '.';
const OBSTACLE_TILE: char = '#';

type Num = i16;
type Pos = I16Vec2;
type Map = Vec<Vec<Tile>>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Empty,
    Obstacle,
}

fn parse_map() -> (Map, Pos) {
    let mut guard_pos = Pos::default();
    let map = include_str!("../in_test.txt")
        .lines()
        .enumerate()
        .map(|(y, line)| line
            .chars()
            .enumerate()
            .map(|(x, c)|
                if c == OBSTACLE_TILE {
                    Tile::Obstacle
                }
                else if c == EMPTY_TILE {
                    Tile::Empty
                }
                else {
                    guard_pos = Pos::new(x as Num, y as Num);
                    Tile::Empty
                }
            ).collect::<Vec<_>>()
        ).collect::<Vec<_>>();

    (map, guard_pos)
}

fn get_tile(map: &Map, pos: &Pos) -> Tile {
    *map.get(pos.y as usize)
        .map(|row| row.get(pos.x as usize).unwrap_or(&Tile::Empty))
        .unwrap_or(&Tile::Empty)
}

fn get_route(map: &Map, guard_pos: &Pos, obstacle_pos: &Option<Pos>) -> (HashSet<(Pos, Pos)>, bool) {
    let mut guard_pos = *guard_pos;
    let mut guard_dir = Pos::new(0, -1);
    let mut is_loop = false;
    let mut visited = HashSet::new();

    while (0..map.len() as Num).contains(&guard_pos.y) && (0..map[0].len() as Num).contains(&guard_pos.x) {
        if !visited.insert((guard_pos, guard_dir)) {
            is_loop = true;
            break;
        }

        let front_pos = guard_pos + guard_dir;
        if get_tile(map, &front_pos) == Tile::Obstacle || obstacle_pos.map(|pos| front_pos == pos).unwrap_or(false) {
            guard_dir.y *= -1;
            std::mem::swap(&mut guard_dir.x, &mut guard_dir.y);
        }
        else {
            guard_pos = front_pos;
        }
    }

    (visited, is_loop)
}

fn solve() -> (Num, Num) {
    let (map, guard_pos) = parse_map();

    let result1 = get_route(&map, &guard_pos, &None).0.len() as Num - 1;
    //let mut result2 = 0;

    // TODO: remove
    //let total = map.len() * map[0].len();
    //let mut i = 0;

    let result2 = iproduct!(0..map.len(), 0..map[0].len()).collect::<Vec<_>>().par_iter().map(|&(x, y)| {
        // TODO: remove
        // if i % 100 == 0 {
        //     println!("{}/{}", i, total);
        // }
        // i += 1;

        let current = Pos::new(x as Num, y as Num);

        if current == guard_pos || get_tile(&map, &current) == Tile::Obstacle {
            return 0;
        }

        //map[y][x] = Tile::Obstacle;
        if get_route(&map, &guard_pos, &Some(current)).1 {
            1
        }
        else {
            0
        }
        //map[y][x] = Tile::Empty;
    }).count() as Num;

    (result1, result2)
}

fn main() {
    let now = Instant::now();
    let (result1, result2) = solve();
    let elapsed = now.elapsed();

    println!("part1: {}", result1);
    println!("part2: {}", result2);
    println!("Elapsed time: {:.2?}", elapsed);

    assert_eq!(result1, 5461);
    assert_eq!(result2, 1836);
}
