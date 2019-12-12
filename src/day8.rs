#[aoc(day8, part1)]
pub fn image_check(input: &str) -> usize {
    input
        .as_bytes()
        .chunks(25 * 6)
        .map(|layer| {
            layer.iter().fold((0, 0, 0), |mut counts, value| {
                match *value as char {
                    '0' => counts.0 = counts.0 + 1,
                    '1' => counts.1 = counts.1 + 1,
                    '2' => counts.2 = counts.2 + 1,
                    _ => (),
                };
                counts
            })
        })
        .map(|stats| (stats.0, stats.1 * stats.2))
        .min_by_key(|stats| stats.0)
        .unwrap()
        .1
}
