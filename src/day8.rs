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

#[aoc(day8, part2)]
pub fn show_image(input: &str) -> String {
    decode_image(input, 25, 6)
}

fn decode_image(input: &str, width: usize, height: usize) -> String {
    let mut image = String::with_capacity(width * height + 2);
    image.push('\n');
    let bytes = input.as_bytes();
    let mut start: usize = 0;
    for _ in 0..height {
        for _ in 0..width {
            let mut i = start;
            while i < input.len() {
                match bytes[i] as char {
                    '0' => {
                        image.push(' ');
                        break;
                    }
                    '1' => {
                        image.push('X');
                        break;
                    }
                    _ => (),
                }
                i = i + width * height;
            }
            start = start + 1;
        }
        image.push('\n');
    }
    image
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        assert_eq!(
            String::from("\n X\nX \n"),
            decode_image("0222112222120000", 2, 2)
        );
    }
}
