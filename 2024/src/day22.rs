use std::collections::{HashMap, HashSet};

use aoc::*;

#[aoc(day22, part1)]
pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|n| n.parse::<u64>().unwrap())
        .map(|n| generate(n, 2000))
        .sum()
}

fn generate(mut num: u64, n: usize) -> u64 {
    for _ in 0..n {
        // let mut alt = num as u16;
        // debug!("num: {}", num);
        let x = num * 64;
        num = num ^ x;
        num = num % 16777216;

        // alt ^= alt << 6;
        // debug!("{} {}", num, alt);

        let x = num / 32;
        num = num ^ x;
        num = num % 16777216;

        // alt ^= alt >> 5;
        // debug!("{} {}", num, alt);

        let x = num * 2048;
        num = num ^ x;
        num = num % 16777216;

        // debug!("{} {}", num * 2048 % 16777216, ((alt as u32) << 11) as u16);
        // alt ^= alt << 11;
        // debug!("{} {}", num, alt);
    }
    num
    // for _ in 0..n {
    //     num ^= num << 6;
    //     num ^= num >> 5;
    //     num ^= num << 11;
    //     debug!("{}", num);
    // }
    // num as u64
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> usize {
    let prices = input
        .lines()
        .map(|n| n.parse::<u64>().unwrap())
        .map(|n| changes(n))
        .collect::<Vec<[(usize, isize); 2000]>>();
    let mut sales = HashMap::new();
    let mut slice = [0; 4];
    let mut seen = HashSet::new();
    for m in prices.iter() {
        seen.clear();
        for i in 0..1995 {
            for j in 0..4 {
                slice[j] = m[i + j].1;
            }
            if seen.insert(slice) {
                let price = m[i + 3].0;
                sales
                    .entry(slice)
                    .and_modify(|k| *k += price)
                    .or_insert(price);
            }
        }
    }
    *sales.values().max().unwrap()
}

fn changes(mut num: u64) -> [(usize, isize); 2000] {
    let mut digit = (num % 10) as isize;
    let mut prices = [(0, 0); 2000];
    for i in 0..2000 {
        let x = num * 64;
        num = num ^ x;
        num = num % 16777216;

        let x = num / 32;
        num = num ^ x;
        num = num % 16777216;

        let x = num * 2048;
        num = num ^ x;
        num = num % 16777216;

        let next = (num % 10) as isize;
        prices[i] = (next as usize, next - digit);
        digit = next;
    }
    prices
}
