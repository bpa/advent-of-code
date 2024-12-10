use aoc::*;
use nom::character::complete;

fn calibration_result(test_value: u64, nums: &[u64]) -> Option<u64> {
    if nums.len() == 1 {
        return if test_value == nums[0] {
            Some(test_value)
        } else {
            None
        };
    }
    let last = nums.last().unwrap();
    let sub = &nums[..nums.len() - 1];
    if test_value % last == 0 && calibration_result(test_value / last, sub).is_some() {
        return Some(test_value);
    }
    if calibration_result(test_value - last, sub).is_some() {
        return Some(test_value);
    }
    None
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(comb(complete::u64))
        .filter_map(|nums| calibration_result(nums[0], &nums[1..]))
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| l.split_ascii_whitespace().collect())
        .map(calibration_with_concatination)
        .filter(|n| *n > 0)
        .sum()
}

fn calibration_with_concatination(mut inputs: Vec<&str>) -> u64 {
    let mut stack = Vec::with_capacity(32);
    inputs[0] = &inputs[0][..inputs[0].len() - 1];
    let nums: Vec<u64> = inputs.iter().map(|s| s.parse::<u64>().unwrap()).collect();
    let test_value = nums[0];
    stack.push((test_value, nums.len() - 1));
    while let Some(next) = stack.pop() {
        if next.1 == 1 {
            match next.0 == nums[next.1] {
                true => return test_value,
                false => continue,
            }
        }
        if next.0 < nums[next.1] {
            continue;
        }
        if let Some(x) = next.0.checked_sub(nums[next.1]) {
            stack.push((x, next.1 - 1));
        }

        if next.0 % nums[next.1] == 0 {
            stack.push((next.0 / nums[next.1], next.1 - 1));
        }

        let cat_str = next.0.to_string();
        if cat_str.ends_with(inputs[next.1]) {
            let next_str = &cat_str[..cat_str.len() - inputs[next.1].len()];
            if !next_str.is_empty() {
                stack.push((next_str.parse::<u64>().unwrap(), next.1 - 1));
            }
        }
    }
    0
}
