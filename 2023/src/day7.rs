fn strength(input: &&str) -> [i32; 6] {
    let mut hand: Vec<u8> = input[..5].bytes().collect();
    hand.sort_unstable();
    let mut copies = [0; 6];
    let mut last = hand[0];
    let mut cnt = 0;
    for i in 1..5 {
        let c = hand[i];
        if c == last {
            cnt += 1;
        } else {
            copies[cnt] += 1;
            last = c;
            cnt = 0;
        }
    }
    copies[cnt] += 1;
    copies[0] = match (copies[1], copies[2], copies[3], copies[4]) {
        (_, _, _, 1) => 6,
        (_, _, 1, _) => 5,
        (1, 1, _, _) => 4,
        (_, 1, _, _) => 3,
        (2, _, _, _) => 2,
        (1, _, _, _) => 1,
        (_, _, _, _) => 0,
    };
    for (i, c) in input[..5].bytes().enumerate() {
        copies[i + 1] = match c {
            b'A' => 13,
            b'K' => 12,
            b'Q' => 11,
            b'J' => 10,
            b'T' => 9,
            b'9' => 8,
            b'8' => 7,
            b'7' => 6,
            b'6' => 5,
            b'5' => 4,
            b'4' => 3,
            b'3' => 2,
            b'2' => 1,
            _ => 0,
        }
    }
    copies
}

fn strength_jacks_wild(input: &&str) -> [i32; 6] {
    let mut hand: Vec<u8> = input[..5].bytes().collect();
    hand.sort_unstable();
    let mut copies = [0; 6];
    let mut last = hand[0];
    let mut cnt = 0;
    let mut jacks = 0;
    for i in 1..5 {
        let c = hand[i];
        if c == last {
            cnt += 1;
        } else if last == b'J' {
            jacks = cnt + 1;
            last = c;
            cnt = 0;
        } else {
            copies[cnt] += 1;
            last = c;
            cnt = 0;
        }
    }
    if hand[4] == b'J' {
        jacks = cnt + 1;
    } else {
        copies[cnt] += 1;
    }
    copies[0] = match (copies[1], copies[2], copies[3], copies[4], jacks) {
        (_, _, _, _, 5) => 6,
        (_, _, _, _, 4) => 6,
        (1, _, _, _, 3) => 6,
        (_, 1, _, _, 2) => 6,
        (_, _, 1, _, 1) => 6,
        (_, _, _, 1, _) => 6,
        (_, _, _, _, 3) => 5,
        (1, _, _, _, 2) => 5,
        (_, 1, _, _, 1) => 5,
        (_, _, 1, _, _) => 5,
        (2, _, _, _, 1) => 4,
        (1, 1, _, _, _) => 4,
        (_, _, _, _, 2) => 3,
        (1, _, _, _, 1) => 3,
        (_, 1, _, _, _) => 3,
        (2, _, _, _, _) => 2,
        (_, _, _, _, 1) => 1,
        (1, _, _, _, _) => 1,
        (_, _, _, _, _) => 0,
    };
    for (i, c) in input[..5].bytes().enumerate() {
        copies[i + 1] = match c {
            b'A' => 13,
            b'K' => 12,
            b'Q' => 11,
            b'T' => 9,
            b'9' => 8,
            b'8' => 7,
            b'7' => 6,
            b'6' => 5,
            b'5' => 4,
            b'4' => 3,
            b'3' => 2,
            b'2' => 1,
            _ => 0,
        }
    }
    copies
}

fn bid((i, input): (usize, &str)) -> u32 {
    input[6..].parse::<u32>().unwrap() * (i as u32 + 1)
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u32 {
    let mut hands: Vec<&str> = input.lines().collect();
    hands.sort_by_cached_key(strength);
    hands.into_iter().enumerate().map(bid).sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u32 {
    let mut hands: Vec<&str> = input.lines().collect();
    hands.sort_by_cached_key(strength_jacks_wild);
    hands.into_iter().enumerate().map(bid).sum()
}
