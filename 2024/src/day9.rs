use std::{
    cmp::{Reverse, min},
    collections::BinaryHeap,
};

use nom::AsChar;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u64 {
    let disk = input.as_bytes();
    let mut end_idx = input.len() + 1;
    let mut end_id: u64 = end_idx as u64 / 2;
    let mut end_count = 0;
    let mut start_idx = 0;
    let mut start_id: u64 = 0;
    let mut pos = 0;
    let mut answer = 0;
    while start_idx < end_idx {
        for _ in 0..disk[start_idx].as_char().to_digit(10).unwrap() {
            answer += start_id * pos;
            pos += 1;
        }
        start_idx += 1;
        let mut free_space = disk[start_idx].as_char().to_digit(10).unwrap();
        while free_space > 0 {
            if end_count == 0 {
                end_idx -= 2;
                end_id -= 1;
                if end_idx <= start_idx {
                    return answer;
                }
                end_count = disk[end_idx].as_char().to_digit(10).unwrap();
            }
            let blocks = min(end_count, free_space);
            free_space -= blocks;
            end_count -= blocks;
            for _ in 0..blocks {
                answer += end_id * pos;
                pos += 1;
            }
        }
        start_idx += 1;
        start_id += 1;
    }
    for _ in 0..end_count {
        answer += end_id * pos;
        pos += 1;
    }
    answer
}

#[derive(Debug)]
struct File {
    pos: usize,
    size: usize,
    id: u64,
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u64 {
    let mut blocks: [BinaryHeap<Reverse<usize>>; 10] =
        std::array::from_fn(|_| BinaryHeap::with_capacity(3072));
    let mut pos = 0;
    let mut id = 0;
    let disk: Vec<File> = input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let size = c.to_digit(10).unwrap() as usize;
            let val = File { pos, size, id };
            pos += size;
            if i % 2 == 1 {
                id += 1;
            }
            val
        })
        .collect();
    for space in disk.iter().skip(1).step_by(2) {
        blocks[space.size].push(Reverse(space.pos));
    }

    let mut ans = 0;
    for file in disk.iter().step_by(2).rev() {
        let next = blocks
            .iter_mut()
            .enumerate()
            .skip(file.size)
            .filter_map(|s| s.1.peek().and_then(|v| Some((s.0, *v))))
            .min_by_key(|o| o.1.0);
        let mut moved_pos = file.pos as u64;
        if let Some(space) = next {
            if space.1.0 < file.pos {
                moved_pos = space.1.0 as u64;
                blocks[space.0].pop();
                let remaining = space.0 - file.size;
                if remaining > 0 {
                    blocks[remaining].push(Reverse(space.1.0 + file.size));
                }
            }
        }
        ans += file.size as u64 * (2 * file.id * moved_pos + (file.size - 1) as u64 * file.id) / 2;
    }
    ans
}
