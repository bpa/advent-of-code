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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_sample() {
        let input = firewall("0: 3\n1: 2\n4: 4\n6: 4");
        assert_eq!(24, severity(&input));
    }
}
