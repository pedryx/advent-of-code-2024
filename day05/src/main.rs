use itertools::Itertools;
use std::{cmp::Ordering, collections::{HashMap, HashSet}, time::Instant};

type PageType = usize;

fn is_valid(update: &[PageType], pages: &[HashSet<PageType>]) -> bool {
    update.iter()
        .enumerate()
        .all(|(i, &page1)| update[i + 1..].iter().all(|page2| !pages[*page2].contains(&page1)))
}

fn sort_update(update: &mut [PageType], pages: &[HashSet<PageType>]) {
    update.sort_by(|a, b| {
        if pages[*a].contains(b) {
            Ordering::Less
        }
        else {
            Ordering::Greater
        }
    });
}

fn solve() -> (PageType, PageType) {
    let (ordering, updates) = include_str!("../in.txt")
        .split("\n\n")
        .next_tuple().unwrap();

    let mut page_to_index = HashMap::new();
    let mut index_to_page = HashMap::new();
    let mut pages = Vec::new();

    for line in ordering.lines() {
        let (a, b) = line.split('|')
            .map(|page| *page_to_index.entry(page).or_insert_with(|| {
                index_to_page.insert(pages.len(), page);
                pages.push(HashSet::new());
                pages.len() - 1
            })).next_tuple().unwrap();
        pages[a].insert(b);
    }

    let (valid_updates, mut invalid_updates): (Vec<_>, Vec<_>) = updates.lines()
        .map(|update| update.split(',').map(|v| page_to_index[v]).collect::<Vec<_>>())
        .partition(|update| is_valid(update, &pages));

    let part1_result = valid_updates.iter()
        .map(|update| index_to_page[&update[update.len() / 2]].parse::<PageType>().unwrap())
        .sum();
    let part2_result = invalid_updates.iter_mut()
        .map(|update| { sort_update(update, &pages); update })
        .map(|update| index_to_page[&update[update.len() / 2]].parse::<PageType>().unwrap())
        .sum();

    (part1_result, part2_result)
}

fn main() {
    let now = Instant::now();
    let (result1, result2) = solve();
    let elapsed = now.elapsed();

    println!("part 1: {}", result1);
    println!("part 2: {}", result2);
    println!("Elapsed time: {:.2?}", elapsed);
}
