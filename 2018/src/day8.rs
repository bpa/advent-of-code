#[aoc(day8, part1)]
fn metadata_sum(input: &str) -> usize {
    node_metadata_sum(&mut input.split_ascii_whitespace().into_iter())
}

fn node_metadata_sum(input: &mut dyn Iterator<Item = &str>) -> usize {
    let nodes: usize = input.next().unwrap().parse().unwrap();
    let metadata: usize = input.next().unwrap().parse().unwrap();
    let mut total = 0;
    for _ in 0..nodes {
        total += node_metadata_sum(input);
    }
    for _ in 0..metadata {
        total += input.next().unwrap().parse::<usize>().unwrap();
    }
    total
}

#[aoc(day8, part2)]
fn root_node_value(input: &str) -> usize {
    node_value(&mut input.split_ascii_whitespace().into_iter())
}

fn node_value(input: &mut dyn Iterator<Item = &str>) -> usize {
    let nodes: usize = input.next().unwrap().parse().unwrap();
    let metadata: usize = input.next().unwrap().parse().unwrap();

    if nodes == 0 {
        return (0..metadata)
            .map(|_| input.next().unwrap().parse::<usize>().unwrap())
            .sum();
    }

    let values: Vec<usize> = (0..nodes).map(|_| node_value(input)).collect();
    let mut value = 0;
    for _ in 0..metadata {
        let index = input.next().unwrap().parse::<usize>().unwrap() - 1;
        value += values.get(index).unwrap_or(&0);
    }
    value
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2() {
        assert_eq!(66, root_node_value("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"));
    }
}
