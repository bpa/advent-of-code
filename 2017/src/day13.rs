use packed_simd::*;

#[aoc_generator(day13)]
pub fn firewall(input: &str) -> Vec<Layer> {
    input
        .lines()
        .map(|l| l.split(": ").collect::<Vec<&str>>())
        .map(|l| {
            Layer::new(
                l[0].parse::<usize>().unwrap(),
                l[1].parse::<usize>().unwrap(),
            )
        })
        .collect()
}

pub struct Layer {
    depth: usize,
    range: usize,
    period: usize,
}

impl Layer {
    fn new(depth: usize, range: usize) -> Self {
        Layer {
            depth,
            range,
            period: match range {
                1 => 1,
                r @ _ => 2 * r - 2,
            },
        }
    }
}

#[aoc(day13, part1)]
pub fn severity(input: &[Layer]) -> usize {
    input
        .iter()
        .map(|l| {
            if l.depth % l.period == 0 {
                l.depth * l.range
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn delay(input: &[Layer]) -> usize {
    assert!(input.len() <= 64);
    let period_values: Vec<u8> = input
        .iter()
        .map(|l| l.period as u8)
        .cycle()
        .take(64)
        .collect();
    let initial_state: Vec<u8> = input
        .iter()
        .map(|l| (l.depth % l.period) as u8)
        .cycle()
        .take(64)
        .collect();

    let one = u8x64::splat(1);
    let periods = u8x64::from_slice_unaligned(period_values.as_slice());
    let mut firewall = u8x64::from_slice_unaligned(initial_state.as_slice());

    let mut delay = 0;
    loop {
        delay = delay + 1;
        firewall = (firewall + one) % periods;
        if firewall.min_element() > 0 {
            return delay;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = firewall("0: 3\n1: 2\n4: 4\n6: 4");
        assert_eq!(24, severity(&input));
    }

    #[test]
    fn part2_sample() {
        let input = firewall("0: 3\n1: 2\n4: 4\n6: 4");
        assert_eq!(10, delay(&input));
    }
}
