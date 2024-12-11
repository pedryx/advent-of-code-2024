use std::time::Instant;

use rustc_hash::FxHashMap;

type StoneNum = i64;
type StoneCountNum = i64;

static BLINK_COUNT: i8 = 75;

fn count_stones(stone: StoneNum, blink_count: i8, cache: &mut FxHashMap<(StoneNum, i8), StoneCountNum>) -> StoneCountNum {
    if let Some(&result) = cache.get(&(stone, blink_count)) {
        return result;
    }
    
    let result = if blink_count == 1 {
        1 + ( stone != 0 && (stone.ilog10() + 1) % 2 == 0) as StoneCountNum
    }
    else if stone == 0 {
        count_stones(1, blink_count - 1, cache)
    }
    else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let pow = (10 as StoneNum).pow(digits / 2);
            let left_stone = stone / pow;
            let right_stone = stone % pow;
    
            count_stones(left_stone, blink_count - 1, cache) + count_stones(right_stone, blink_count - 1, cache)
        }
        else {
            count_stones(stone * 2024, blink_count - 1, cache)
        }
    };

    cache.insert((stone, blink_count), result);
    result
}

fn main() {
    let mut cache = FxHashMap::default();

    let now = Instant::now();
    let stone_count = include_str!("../in.txt")
        .split_ascii_whitespace()
        .map(|token| count_stones(token.parse().unwrap(), BLINK_COUNT, &mut cache)).sum::<StoneCountNum>();
    let elapsed = now.elapsed();

    println!("part1: {}", stone_count);
    println!("elapsed: {:.2?}", elapsed);
}
