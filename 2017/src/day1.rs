#[aoc_generator(day1)]
pub fn to_digits(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn captcha(input: &[u32]) -> u32 {
    let count = input
        .windows(2)
        .map(|win| if win[0] == win[1] { win[0] } else { 0 })
        .sum();
    if input.first() == input.last() {
        count + input.last().unwrap()
    } else {
        count
    }
}

#[aoc(day1, part2)]
pub fn captcha_circular(input: &[u32]) -> u32 {
    let half = input.len() / 2;
    let mut sum = 0;
    for c in 0..input.len() {
        if input[c] == input[(c + half) % input.len()] {
            sum += input[c];
        }
    }
    sum
}
