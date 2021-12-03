#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let gamma = usize::from_str_radix(&gamma(&input.lines().collect::<Vec<&str>>()), 2).unwrap();
    gamma * (gamma ^ ((1 << width) - 1))
}

fn gamma(input: &Vec<&str>) -> String {
    let lines = (input.len() + 1) / 2;
    input
        .iter()
        .map(|n| {
            n.bytes()
                .map(|c| if c == '1' as u8 { 1 } else { 0 })
                .collect::<Vec<usize>>()
        })
        .reduce(|a, b| {
            a.iter()
                .zip(b.iter())
                .map(|(l, r)| l + r)
                .collect::<Vec<usize>>()
        })
        .unwrap()
        .iter()
        .map(|n| if n >= &lines { '1' } else { '0' })
        .collect::<String>()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let mut candidates = input.lines().collect::<Vec<&str>>();

    for i in 0..width {
        let gamma = gamma(&candidates);
        let desired = gamma.chars().nth(i).unwrap();
        candidates = candidates
            .into_iter()
            .filter(|c| c.chars().nth(i).unwrap() == desired)
            .collect::<Vec<&str>>();
        if candidates.len() == 1 {
            break;
        }
    }

    let a = candidates.into_iter().next().unwrap();
    let mut candidates = input.lines().collect::<Vec<&str>>();

    for i in 0..width {
        let gamma = gamma(&candidates);
        let desired = gamma.chars().nth(i).unwrap();
        candidates = candidates
            .into_iter()
            .filter(|c| c.chars().nth(i).unwrap() != desired)
            .collect::<Vec<&str>>();
        if candidates.len() == 1 {
            break;
        }
    }

    let b = candidates.into_iter().next().unwrap();
    let a = usize::from_str_radix(a, 2).unwrap();
    let b = usize::from_str_radix(b, 2).unwrap();

    a * b
}
