#[aoc_generator(day19)]
fn pipes(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

enum Dir {
    N,
    E,
    S,
    W,
}

#[aoc(day19, part1)]
fn letters_along_the_way(pipes: &Vec<Vec<char>>) -> String {
    let mut x = pipes[0].iter().position(|&c| c == '|').unwrap() as isize;
    let mut y = 0isize;
    let mut seen = Vec::new();
    let mut direction = Dir::S;

    loop {
        match pipes[y as usize][x as usize] {
            '|' => {}
            '-' => {}
            '+' => {
                direction = match direction {
                    Dir::N | Dir::S => {
                        if x < 1 || "| +".contains(pipes[y as usize][(x - 1) as usize]) {
                            Dir::E
                        } else {
                            Dir::W
                        }
                    }
                    _ => {
                        if y < 1 || "- +".contains(pipes[(y - 1) as usize][x as usize]) {
                            Dir::S
                        } else {
                            Dir::N
                        }
                    }
                }
            }
            ' ' => break,
            c @ _ => seen.push(c),
        }

        match direction {
            Dir::N => y -= 1,
            Dir::E => x += 1,
            Dir::S => y += 1,
            Dir::W => x -= 1,
        }

        if x < 0 || x >= pipes[0].len() as isize || y < 0 || y >= pipes.len() as isize {
            break;
        }
    }

    seen.into_iter().collect()
}

#[aoc(day19, part2)]
fn steps(pipes: &Vec<Vec<char>>) -> usize {
    let mut x = pipes[0].iter().position(|&c| c == '|').unwrap() as isize;
    let mut y = 0isize;
    let mut step = 0;
    let mut direction = Dir::S;

    loop {
        match pipes[y as usize][x as usize] {
            '+' => {
                direction = match direction {
                    Dir::N | Dir::S => {
                        if x < 1 || "| +".contains(pipes[y as usize][(x - 1) as usize]) {
                            Dir::E
                        } else {
                            Dir::W
                        }
                    }
                    _ => {
                        if y < 1 || "- +".contains(pipes[(y - 1) as usize][x as usize]) {
                            Dir::S
                        } else {
                            Dir::N
                        }
                    }
                }
            }
            ' ' => break,
            _ => {}
        }

        match direction {
            Dir::N => y -= 1,
            Dir::E => x += 1,
            Dir::S => y += 1,
            Dir::W => x -= 1,
        }

        step += 1;

        if x < 0 || x >= pipes[0].len() as isize || y < 0 || y >= pipes.len() as isize {
            break;
        }
    }

    step
}

#[cfg(test)]
mod test {
    use super::*;

    const DIAGRAM: &str = "    |          
    |  +--+    
    A  |  C    
F---|----E|--+ 
    |  |  |  D 
    +B-+  +--+";

    #[test]
    fn part1() {
        assert_eq!(letters_along_the_way(&pipes(DIAGRAM)), "ABCDEF");
    }

    #[test]
    fn part2() {
        assert_eq!(steps(&pipes(DIAGRAM)), 38);
    }
}
