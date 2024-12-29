#[aoc(day25, part1)]
pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    loop {
        let line = lines.next().unwrap();
        let is_lock = line.contains("#");
        let mut shape = [0; 5];
        for _ in 0..5 {
            let line = lines.next().unwrap();
            for (i, v) in line.chars().enumerate() {
                if v == '#' {
                    shape[i] += 1;
                }
            }
        }
        lines.next();
        if is_lock {
            locks.push(shape);
        } else {
            keys.push(shape);
        }
        if lines.next().is_none() {
            break;
        }
    }
    let mut answer = 0;
    for k in &keys {
        for l in &locks {
            if k.iter().zip(l).map(|(a, b)| a + b).max() <= Some(5) {
                answer += 1;
            }
        }
    }
    answer
}
