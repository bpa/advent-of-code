use std::mem::swap;

#[aoc_generator(day12)]
fn parse_input(input: &str) -> (Vec<bool>, [bool; 32]) {
    let mut lines = input.lines();
    let initial_state_line = lines.next().unwrap();
    let mut pots = initial_state_line.chars();

    let mut initial_state = Vec::new();
    pots.nth(14);
    for pot in pots {
        initial_state.push(pot == '#');
    }

    lines.next().unwrap();
    let mut mapping = [false; 32];
    for line in lines {
        let mut chars = line.chars();
        let mut key = 0;
        for _ in 0..5 {
            key *= 2;
            if chars.next().unwrap() == '#' {
                key += 1;
            }
        }
        if chars.nth(4).unwrap() == '#' {
            mapping[key] = true;
        }
    }
    (initial_state, mapping)
}

#[aoc(day12, part1)]
fn after20(input: &(Vec<bool>, [bool; 32])) -> isize {
    after_x(20, input)
}

#[aoc(day12, part2)]
fn after_50_b(input: &(Vec<bool>, [bool; 32])) -> isize {
    after_x(50_000_000_000, input)
}

fn after_x(iterations: usize, (initial_state, mapping): &(Vec<bool>, [bool; 32])) -> isize {
    let mut current = initial_state.clone();
    let mut next_gen = Vec::with_capacity(current.len());
    let mut min: isize = 0;
    for it in 0..iterations {
        next_gen.clear();
        let mut plants = current.iter();
        let mut pattern = 0;
        let last = min;
        min -= 3;
        loop {
            match plants.next() {
                Some(plant) => {
                    min += 1;
                    pattern = pattern & 0xF;
                    pattern *= 2;
                    if *plant {
                        pattern += 1;
                    }
                    if mapping[pattern] {
                        next_gen.push(true);
                        break;
                    }
                }
                None => break,
            }
        }

        for plant in plants {
            pattern = pattern & 0xF;
            pattern *= 2;
            if *plant {
                pattern += 1;
            }
            next_gen.push(mapping[pattern])
        }

        while pattern > 0 {
            pattern = pattern & 0xF;
            pattern *= 2;
            next_gen.push(mapping[pattern])
        }

        if current == next_gen {
            min += (min - last) * (iterations - it - 1) as isize;
            break;
        }
        swap(&mut current, &mut next_gen);
    }
    current
        .iter()
        .enumerate()
        .filter_map(|(i, &plant)| plant.then_some(i as isize + min))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            325,
            after20(&parse_input(
                "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"
            ))
        )
    }
}
