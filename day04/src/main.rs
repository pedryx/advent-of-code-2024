use std::collections::HashMap;
use aho_corasick::AhoCorasick;

type InputGrid = Vec<Vec<u8>>;
type Coord = (usize, usize);

fn find_occurrences_in_line(input: &InputGrid, ac: &AhoCorasick, coords: Vec<Coord>) -> Vec<Coord> {
    let line = &coords.iter().map(|&(x, y)| input[x][y]).collect::<Vec<_>>();
    ac.find_overlapping_iter(line)
        .map(|occurence| coords[occurence.start() + 1])
        .collect()
}

fn find_occurrences_in_direction<'a>(
    input: &'a InputGrid,
    ac: &'a AhoCorasick,
    dir_coords: impl Iterator<Item = impl DoubleEndedIterator<Item = Coord>> + Clone + 'a
) -> impl Iterator<Item = Coord> + 'a {
    dir_coords.clone().flat_map(|coords| find_occurrences_in_line(input, ac, coords.collect()))
}

fn find_occurrences_in_grid(patterns: &[&'static str], input: &InputGrid, diag_only: bool) -> Vec<Coord> {
    let len = input.len();
    let ac = AhoCorasick::new(patterns).unwrap();

    let diag_coords = (0..len).map(|x| (0..len - x).map(|y| (x + y, y)).collect::<Vec<_>>())
        .chain((1..len).map(|y| (0..len - y).map(|x| (x, x + y)).collect::<Vec<_>>()))
        .map(|coords| coords.into_iter());
    let diag_results = find_occurrences_in_direction(input, &ac, diag_coords.clone());

    let anti_diag_coords = diag_coords.map(|d| d.map(|(x, y)| (len - x - 1, y)));
    let anti_diag_results = find_occurrences_in_direction(input, &ac, anti_diag_coords);

    let results = diag_results.chain(anti_diag_results);

    if diag_only {
        return results.collect();
    }

    let orthogonal_coords = (0..len).flat_map(|i| 
        vec![
            (0..len).map(move |j| (i, j)).collect(),
            (0..len).map(move |j| (j, i)).collect(),
        ]
    ).map(|coords: Vec<_>| coords.into_iter());
    results.chain(find_occurrences_in_direction(input, &ac, orthogonal_coords)).collect()
}

fn main() {
    let input: InputGrid = include_str!("../in.txt")
        .lines()
        .map(|l| l.bytes().collect())
        .collect();
    
    let result_part1 = find_occurrences_in_grid(&["XMAS", "SAMX"], &input, false).len();
    let result_part2 = find_occurrences_in_grid(&["MAS", "SAM"], &input, true).iter()
        .fold(HashMap::new(), |mut counter, coord| {
            *counter.entry(coord).or_insert(0) += 1;
            counter
        }).iter()
        .filter(|&(_, &count)| count >= 2)
        .count();

    println!("part1: {:?}", result_part1);
    println!("part1: {:?}", result_part2);
}
