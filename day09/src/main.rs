use std::{time::Instant};

type Num = u64;

#[derive(Default)]
struct State {
    input_index: usize,
    input_back_index: usize,
    current_back_block_count: Num,
    position_in_disc: Num,
    update_part1: bool
}

impl State {
    fn new(input: &[u8]) -> Self {
        Self {
            input_back_index: input.len() - 1,
            current_back_block_count: input[input.len() - 1] as Num,
            update_part1: true,
            ..Default::default()
        }
    }
}

fn calc_checksum(block_count: Num, position: Num, first_block_id: Num) -> Num {
    block_count * (2 * position + block_count - 1) * first_block_id / 2
}

fn process_part1_front(input: &[u8], state: &State) -> Num {
    if !state.update_part1 {
        return 0;
    }

    let block_count = if state.input_index == state.input_back_index {
        state.current_back_block_count
    } else {
        input[state.input_index] as Num
    };

    calc_checksum(block_count, state.position_in_disc, state.input_index as Num / 2)
}

fn process_part1_back(input: &[u8], state: &mut State) -> Num {
    if !state.update_part1 {
        return 0;
    }

    let mut checksum = 0;
    let mut position = state.position_in_disc;
    let mut free_block_count = input[state.input_index] as Num;

    while free_block_count > 0 {
        let block_id = state.input_back_index as Num / 2;
        let block_count = free_block_count.min(state.current_back_block_count);

        checksum += (block_count * (2 * position + (block_count - 1)) * block_id) / 2;
        position += block_count;

        if state.current_back_block_count > free_block_count {
            state.current_back_block_count -= free_block_count;
            break;
        }

        free_block_count -= state.current_back_block_count;
        state.input_back_index -= 2;
        state.current_back_block_count = input[state.input_back_index] as Num;

        if state.input_back_index < state.input_index {
            break;
        }
    }

    checksum
}

fn process_part2_back(input: &mut [u8], state: &State, block_count_map: &mut [Vec<Num>]) -> Num {
    let mut checksum = 0;
    let mut position = state.position_in_disc;
    let mut free_block_count = input[state.input_index] as Num;

    while free_block_count > 0 {
        let mut block_id = 0;
        let mut back_block_count = 0;

        for block_count in 1..=free_block_count {
            let Some(&current_block_id) = block_count_map[block_count as usize].last() else {
                continue;
            };

            if current_block_id > block_id && state.input_index < 2 * current_block_id as usize {
                block_id = current_block_id;
                back_block_count = block_count;
            }
        }
        block_count_map[back_block_count as usize].pop();                

        if block_id == 0 {
            break;
        }
        
        checksum += calc_checksum(back_block_count, position, block_id);
        position += back_block_count;
        input[block_id as usize * 2] = 0;
        free_block_count -= back_block_count;
    }

    checksum
}

fn solve() -> (Num, Num) {
    let input = include_str!("../in.txt")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();
    let mut input_part2 = input.clone();

    let mut block_count_map = vec![Vec::new(); 10];
    input.iter()
        .enumerate()
        .filter_map(|(i, block_count)| if i % 2 == 0 { Some((i / 2, block_count)) } else { None })
        .for_each(|(id, &block_count)| block_count_map[block_count as usize].push(id as Num));

    let mut state = State::new(&input);
    let (mut checksum_part1, mut checksum_part2) = (0, 0);

    for input_index in 0..input.len() {
        state.input_index = input_index;
        state.update_part1 &= state.input_back_index >= input_index;

        if input_index % 2 == 0 {
            checksum_part1 += process_part1_front(&input, &state);
            checksum_part2 += calc_checksum(
                input_part2[input_index] as Num,
                state.position_in_disc,
                input_index as Num / 2
            );
        }
        else {
            checksum_part1 += process_part1_back(&input, &mut state);
            checksum_part2 += process_part2_back(&mut input_part2, &state, &mut block_count_map);
        }

        state.position_in_disc += input[input_index] as Num
    }

    (checksum_part1, checksum_part2)
}

fn main() {
    let now = Instant::now();
    let (part1, part2) = solve();
    let elapsed = now.elapsed();

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    println!("elapsed: {:.2?}", elapsed);
}
