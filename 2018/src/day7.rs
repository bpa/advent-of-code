use std::cmp::max;
use std::collections::BTreeSet;
use std::mem::swap;

#[aoc_generator(day7)]
fn dependencies(input: &str) -> Vec<Step> {
    let mut steps = vec![Step::default(); 26];
    for line in input.lines() {
        let mut b = line.bytes();
        let required = b.nth(5).unwrap() - 'A' as u8;
        let step = b.nth(30).unwrap() - 'A' as u8;
        steps[required as usize].dependents.push(step);
        steps[step as usize].dependencies += 1;
    }
    steps
}

#[derive(Clone, Default)]
struct Step {
    dependents: Vec<u8>,
    dependencies: usize,
}

#[aoc(day7, part1)]
fn part_order(input: &Vec<Step>) -> String {
    let mut steps = input.clone();
    let mut available: BTreeSet<usize> = steps
        .iter()
        .enumerate()
        .filter(|(_, s)| s.dependencies == 0)
        .map(|(i, _)| i)
        .collect();

    let mut order = Vec::with_capacity(26);
    let mut dependents: Vec<u8> = Vec::new();
    while let Some(step) = available.pop_first() {
        order.push(step);
        swap(&mut dependents, &mut steps[step].dependents);
        for dep in &dependents {
            steps[*dep as usize].dependencies -= 1;
            if steps[*dep as usize].dependencies == 0 {
                available.insert(*dep as usize);
            }
        }
    }

    String::from_utf8(order.iter().map(|step| *step as u8 + 'A' as u8).collect()).unwrap()
}

#[aoc(day7, part2)]
fn build_time(input: &Vec<Step>) -> usize {
    let mut steps = input.clone();
    let mut available: BTreeSet<usize> = steps
        .iter()
        .enumerate()
        .filter(|(_, s)| s.dependencies == 0)
        .map(|(i, _)| i)
        .collect();

    let mut workers = BTreeSet::new();
    let mut dependents: Vec<u8> = Vec::new();
    let mut seconds = 0;

    while !(available.is_empty() && workers.is_empty()) {
        while workers.len() < 5 {
            match available.pop_first() {
                Some(step) => {
                    workers.insert((seconds + 61 + step, step));
                }
                None => break,
            }
        }
        if let Some((time, step)) = workers.pop_first() {
            seconds = max(seconds, time);
            swap(&mut dependents, &mut steps[step].dependents);
            for dep in &dependents {
                steps[*dep as usize].dependencies -= 1;
                if steps[*dep as usize].dependencies == 0 {
                    available.insert(*dep as usize);
                }
            }
        }
    }

    seconds
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref INPUT: Vec<Step> = vec![
            Step {
                dependencies: 1,
                dependents: vec![1, 3]
            },
            Step {
                dependencies: 1,
                dependents: vec![4]
            },
            Step {
                dependencies: 0,
                dependents: vec![0, 5]
            },
            Step {
                dependencies: 1,
                dependents: vec![4]
            },
            Step {
                dependencies: 3,
                dependents: vec![]
            },
            Step {
                dependencies: 1,
                dependents: vec![4]
            },
        ];
    }

    #[test]
    fn part1() {
        assert_eq!("CABDFE", part_order(&INPUT))
    }

    #[test]
    fn part2() {
        assert_eq!(253, build_time(&INPUT))
    }
}
