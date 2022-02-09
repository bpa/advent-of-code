use cpu::Intcode;
use std::iter::empty;
use std::num::ParseIntError;

#[aoc_generator(day17)]
fn read_memory(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|num| num.parse::<isize>()).collect()
}

#[aoc(day17, part1)]
pub fn part1(m: &Vec<isize>) -> usize {
    let mut answer = 0;
    let image = Intcode::new(m.clone(), &mut empty()).collect::<Vec<isize>>();
    let width = image.iter().position(|c| *c == 10isize).unwrap() + 1;
    let height = image.len() / width;
    for y in 1..height - 1 {
        let o = y * width;
        for x in 1..width - 1 {
            let i = o + x;
            if image[i - 1] == 35
                && image[i] == 35
                && image[i + 1] == 35
                && image[i - width] == 35
                && image[i + width] == 35
            {
                answer += x * y;
            }
        }
    }
    answer
}

#[derive(Debug)]
struct Bot {
    image: Vec<isize>,
    width: usize,
    height: usize,
    x: isize,
    y: isize,
    d: usize,
}

impl Bot {
    fn new(image: Vec<isize>) -> Bot {
        let width = image.iter().position(|c| *c == 10isize).unwrap() + 1;
        let height = (image.len() + 1) / width;
        let bot = "^<>v".bytes().map(|c| c as isize).collect::<Vec<isize>>();
        let bot = image.iter().position(|c| bot.contains(c)).unwrap();
        let x = (bot % width) as isize;
        let y = (bot / width) as isize;
        let d = match char::from_u32(image[bot] as u32).unwrap() {
            '^' => 0,
            '>' => 1,
            'v' => 2,
            '<' => 3,
            _ => unreachable!(),
        };
        Bot {
            image,
            width,
            height,
            x,
            y,
            d,
        }
    }

    fn find_moves(&mut self) -> (String, Vec<String>) {
        let mut moves = Vec::new();
        while let Some(t) = self.turn() {
            let mut steps = 0;
            while self.move_one() {
                steps += 1;
            }
            moves.push((t, steps));
        }
        for a in 1..=5 {
            for b in 1..=5 {
                for c in 1..=5 {
                    if let Some(answer) = test_move(vec![a, b, c], &moves) {
                        return answer;
                    }
                }
            }
        }
        panic!("Can't find answer");
    }

    fn turn(&mut self) -> Option<char> {
        let turns: Vec<(isize, isize)> = match self.d {
            0 => vec![(-1, 0), (1, 0)],
            1 => vec![(0, -1), (0, 1)],
            2 => vec![(1, 0), (-1, 0)],
            3 => vec![(0, 1), (0, -1)],
            _ => unreachable!(),
        };
        for (l, t) in turns.iter().zip(vec!['L', 'R']) {
            if self.tile(l.0 + self.x, l.1 + self.y) == '#' {
                if t == 'L' {
                    self.d = (self.d + 3) % 4;
                } else {
                    self.d = (self.d + 1) % 4;
                }
                return Some(t);
            }
        }
        None
    }

    fn move_one(&mut self) -> bool {
        match self.d {
            0 => {
                if self.y == 0 || self.tile(self.x, self.y - 1) != '#' {
                    return false;
                } else {
                    self.y -= 1;
                }
            }
            1 => {
                if self.tile(self.x + 1, self.y) != '#' {
                    return false;
                } else {
                    self.x += 1;
                }
            }
            2 => {
                if self.y as usize == self.height - 1 || self.tile(self.x, self.y + 1) != '#' {
                    return false;
                } else {
                    self.y += 1;
                }
            }
            3 => {
                if self.x == 0 || self.tile(self.x - 1, self.y) != '#' {
                    return false;
                } else {
                    self.x -= 1;
                }
            }
            _ => unreachable!(),
        }
        true
    }

    fn tile(&self, x: isize, y: isize) -> char {
        if x < 0 || y < 0 {
            return '.';
        }
        char::from_u32(
            *self
                .image
                .get(y as usize * self.width + x as usize)
                .unwrap_or(&('.' as isize)) as u32,
        )
        .unwrap()
    }
}

fn test_move(sizes: Vec<usize>, moves: &Vec<(char, usize)>) -> Option<(String, Vec<String>)> {
    let mut program = Vec::with_capacity(10);
    let mut tokens = vec![Vec::new(), Vec::new(), Vec::new()];
    let mut ind = 0;
    for (i, s) in sizes.iter().enumerate() {
        for _ in 0..*s {
            tokens[i].push(moves[ind]);
            ind += 1;
        }
        program.push(i);
        consume(&tokens, &mut program, &mut ind, &moves);
    }
    if ind == moves.len() {
        let program = program
            .iter()
            .map(|p| char::from_u32(*p as u32 + 'A' as u32).unwrap())
            .collect::<Vec<char>>()
            .into_iter()
            .intersperse(',')
            .collect::<String>();
        let tokens = tokens
            .iter()
            .map(|token| {
                token
                    .iter()
                    .map(|t| format!("{},{}", t.0, t.1))
                    .collect::<Vec<String>>()
                    .into_iter()
                    .intersperse(String::from(","))
                    .collect::<String>()
            })
            .collect::<Vec<String>>();
        return Some((program, tokens));
    }
    None
}

fn consume(
    tokens: &Vec<Vec<(char, usize)>>,
    program: &mut Vec<usize>,
    ind: &mut usize,
    moves: &Vec<(char, usize)>,
) {
    let mut moved = true;
    while moved && *ind < moves.len() {
        moved = false;
        for (i, token) in tokens.iter().enumerate() {
            if token.len() > 0 && matches(token, *ind, moves) {
                program.push(i);
                *ind += token.len();
                moved = true;
            }
        }
    }
}

fn matches(token: &Vec<(char, usize)>, ind: usize, moves: &Vec<(char, usize)>) -> bool {
    for (j, tok) in token.iter().enumerate() {
        if moves[ind + j] != *tok {
            return false;
        }
    }
    true
}

#[aoc(day17, part2)]
pub fn part2auto(m: &Vec<isize>) -> isize {
    let image = Intcode::new(m.clone(), &mut empty()).collect::<Vec<isize>>();
    let mut bot = Bot::new(image);
    let (routines, functions) = bot.find_moves();
    let mut input = Vec::new();
    input.extend(routines.bytes());
    input.push(10u8);
    for f in functions {
        input.extend(f.bytes());
        input.push(10u8);
    }
    input.push('n' as u8);
    input.push(10u8);

    let mut memory = m.clone();
    memory[0] = 2;
    Intcode::new(memory, &mut input.into_iter().map(|b| b as isize))
        .last()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    fn image() -> Vec<isize> {
        "
#######...#####
#.....#...#...#
#.....#...#...#
......#...#...#
......#...###.#
......#.....#.#
^########...#.#
......#.#...#.#
......#########
........#...#..
....#########..
....#...#......
....#...#......
....#...#......
....#####......"
            .bytes()
            .skip(1)
            .map(|c| c as isize)
            .collect::<Vec<isize>>()
    }

    #[test]
    fn new_bot() {
        let bot = Bot::new(image());
        assert_eq!(bot.width, 16);
        assert_eq!(bot.height, 15);
        assert_eq!(bot.x, 0);
        assert_eq!(bot.y, 6);
        assert_eq!(bot.d, 0);
    }

    #[test]
    fn find_moves() {
        let mut bot = Bot::new(image());
        bot.find_moves();
        assert_eq!(1, 2);
    }
}
