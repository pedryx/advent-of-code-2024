use std::{array::from_fn, io::BufRead, time::Instant};
use glam::IVec2;
use itertools::Itertools;

static DIRS4: [IVec2; 4] = [IVec2::new(1, 0), IVec2::new(-1, 0), IVec2::new(0, 1), IVec2::new(0, -1)];
static DIRS8: [IVec2; 8] = [
    IVec2::new(-1, -1), IVec2::new(0, -1), IVec2::new(1, -1),
    IVec2::new(-1, 0), IVec2::new(1, 0),
    IVec2::new(-1, 1), IVec2::new(0, 1), IVec2::new(1, 1),
];

fn index_to_pos(index: usize, size: usize) -> IVec2 {
    IVec2::new((index % size) as i32, (index / size) as i32)
}

fn get_neighbor(garden: &[u8], region_flower: u8, size: usize, current: usize, dir: IVec2) -> Option<usize> {
    let pos = index_to_pos(current, size) + dir;

    ((0..size as i32).contains(&pos.x) && (0..size as i32).contains(&pos.y))
        .then(|| pos.y as usize * size + pos.x as usize)
        .and_then(|index| (garden[index] == region_flower).then(|| index))
}

fn get_corner_count(garden: &[u8], size: usize, region_flower: u8, corner_map: &[i32; 256], index: usize) -> i32 {
    let corner_map_index = (0..DIRS8.len())
        .map(|i| if get_neighbor(garden, region_flower, size, index, DIRS8[i]).is_some() { 1 << i } else { 0 })
        .sum::<usize>();

    corner_map[corner_map_index]
}

fn count_stats(garden: &[u8], visited: &mut [bool], corner_map: &[i32; 256], size: usize, start: usize) -> IVec2 {
    if visited[start as usize] {
        return IVec2::ZERO;
    }

    let region_flower = garden[start];
    let mut frontier = vec![start];
    let mut area = 0;
    let mut perimeter = 0;
    let mut side_count = 0;
    visited[start] = true;

    while let Some(current) = frontier.pop() {
        // number of sides is same as number of corners
        side_count += get_corner_count(garden, size, region_flower, corner_map, current);
        area += 1;

        for dir in DIRS4 {
            let Some(neighbor) = get_neighbor(garden, region_flower, size, current, dir) else {
                perimeter += 1;
                continue;
            };
            if visited[neighbor] {
                continue;
            }

            visited[neighbor] = true;
            frontier.push(neighbor);
        }
    }

    IVec2::new(area * perimeter, area * side_count)
}

fn solve() -> (i32, i32) {
    // precomputed mapping from 3x3 window of tiles into number of corners
    let corner_map: [i32; 256] = from_fn(|neighbors| {
            let [top_left, top, top_right, left, right, bottom_left, bottom, bottom_right] 
                = from_fn(|i| neighbors & (1 << i) > 0);

            let top_left_corner = (!left && !top) || (!top_left && left && top);
            let top_right_corner = (!right && !top) || (!top_right && right && top);
            let bottom_left_corner = (!left && !bottom) || (!bottom_left && left && bottom);
            let bottom_right_corner = (!right && !bottom) || (!bottom_right && right && bottom);

            top_left_corner as i32 + top_right_corner as i32 + bottom_left_corner as i32 + bottom_right_corner as i32
        }
    );

    let input = include_bytes!("../in.txt");
    let size = input.lines().next().unwrap().unwrap().len();
    let garden: &[u8] = &input
        .into_iter()
        .filter(|&&b| b != b'\n')
        .copied()
        .collect_vec();
    let mut visited = vec![false; garden.len()];

    let result = garden
        .iter()
        .enumerate()
        .map(|(i, _)| count_stats(garden, &mut visited, &corner_map, size, i))
        .sum::<IVec2>();

    (result.x, result.y)
}

fn main() {
    let now = Instant::now();
    let (result1, result2) = solve();
    let elapsed = now.elapsed();

    println!("part1: {}", result1);
    println!("part2: {}", result2);
    println!("elapsed: {:.2?}", elapsed);
}
