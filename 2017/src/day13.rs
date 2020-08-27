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

struct FirewallLayer {
    pos: usize,
    period: usize,
}

#[aoc(day13, part2)]
pub fn delay(input: &[Layer]) -> usize {
    let mut firewall: Vec<FirewallLayer> = input
        .iter()
        .map(|l| FirewallLayer {
            pos: l.depth % l.period,
            period: l.period,
        })
        .collect();
    let mut delay = 0;
    loop {
        delay = delay + 1;
        let mut caught = false;
        for layer in firewall.iter_mut() {
            let next = (layer.pos + 1) % layer.period;
            layer.pos = next;
            if next == 0 {
                caught = true;
            }
        }
        if !caught {
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
