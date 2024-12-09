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
        // .filter(|l| l.starts_with("7290:"))
        .map(|l| l.split_ascii_whitespace().collect())
        .map(calibration_with_concatination)
        .filter(|n| *n > 0)
        .inspect(|v| debug!("Success: {}", v))
        .sum()
}

fn calibration_with_concatination(mut inputs: Vec<&str>) -> u64 {
    let mut stack = Vec::with_capacity(4096);
    inputs[0] = &inputs[0][..inputs[0].len() - 1];
    let nums: Vec<u64> = inputs.iter().map(|s| s.parse::<u64>().unwrap()).collect();
    let test_value = nums[0];
    stack.push((nums[1], 2));
    // debug!("{:?}", nums);
    while let Some(next) = stack.pop() {
        // debug!("{:?}", next);
        if next.0 > test_value {
            continue;
        }
        if nums.len() == next.1 {
            match test_value == next.0 {
                true => return test_value,
                false => continue,
            }
        }
        let mut cat_str = next.0.to_string();
        cat_str.push_str(inputs[next.1]);
        stack.push((cat_str.parse::<u64>().unwrap(), next.1 + 1));
        stack.push((next.0 * nums[next.1], next.1 + 1));
        stack.push((next.0 + nums[next.1], next.1 + 1));
        // let cat_value = cat_str.parse::<u64>().unwrap();
        // let this_value = nums[0].parse::<u64>().unwrap();
        // calibration_with_concatination(test_value, &cat_str, cat_value, sub)
        //     || calibration_with_concatination(test_value, nums[0], this_value * next_value, sub)
        //     || calibration_with_concatination(test_value, nums[0], this_value + next_value, sub)
    }
    0
}
