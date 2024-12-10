use std::cmp::min;

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

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u64 {
    let mut free_space = Vec::with_capacity(10_240);
    let mut files = Vec::with_capacity(10_240);
    let mut it = input.chars().map(|c| c.to_digit(10).unwrap() as u64);
    let mut pos = it.next().unwrap();
    let mut id = 0;
    files.push((0, pos, id));
    while let Some(b) = it.next() {
        free_space.push((pos, b));
        pos += b;
        id += 1;
        let next = it.next().unwrap();
        files.push((pos, next, id));
        pos += next;
    }
    for i in (0..files.len()).rev() {
        let file = &mut files[i];
        for j in free_space.iter_mut() {
            if j.0 > file.0 {
                break;
            }
            if j.1 >= file.1 {
                file.0 = j.0;
                j.0 += file.1;
                j.1 -= file.1;
                break;
            }
        }
    }
    files
        .iter()
        .map(|f| f.1 * (2 * f.0 * f.2 + (f.1 - 1) * f.2) / 2)
        .sum()
}
