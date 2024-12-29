use std::collections::{HashMap, hash_map::Entry};

use aoc::*;

struct KeyPad {
    sequence_cache: HashMap<(char, char), Vec<char>>,
    press_count_cache: HashMap<(char, char, usize), usize>,
    numpad: Grid<char>,
    dirpad: Grid<char>,
    robots: usize,
}

impl KeyPad {
    fn new(robots: usize) -> Self {
        Self {
            sequence_cache: HashMap::new(),
            press_count_cache: HashMap::new(),
            numpad: Grid::from("789\n456\n123\n#0A"),
            dirpad: Grid::from("#^A\n<v>"),
            robots,
        }
    }

    fn enter_code(&mut self, code: &str) -> usize {
        let code_id = code[..code.len() - 1].parse::<usize>().unwrap();
        let mut count = 0;
        let mut curr = 'A';
        for c in code.chars() {
            count += self.press_button(curr, c, self.robots);
            curr = c;
        }
        count * code_id
    }

    fn press_button(&mut self, start: char, end: char, robots: usize) -> usize {
        if let Some(presses) = self.press_count_cache.get(&(start, end, robots)) {
            return *presses;
        }
        if robots == 0 {
            let count = self.key_sequence(start, end).len();
            self.press_count_cache.insert((start, end, robots), count);
            return count;
        }
        let mut count = 0;
        let mut curr = 'A';
        // TODO: How do you get rid of this clone and make the borrow checker happy?
        let seq = self.key_sequence(start, end).clone();
        for c in seq {
            count += self.press_button(curr, c, robots - 1);
            curr = c;
        }
        self.press_count_cache.insert((start, end, robots), count);
        count
    }

    fn key_sequence(&mut self, start: char, end: char) -> &Vec<char> {
        match self.sequence_cache.entry((start, end)) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => {
                let mut buttons = Vec::new();
                if start != end {
                    let pad = if start.is_digit(10) || end.is_digit(10) {
                        &self.numpad
                    } else {
                        &self.dirpad
                    };
                    let k0 = pad.index_of(start).unwrap();
                    let k1 = pad.index_of(end).unwrap();
                    let vertical_first = match (start, end) {
                        ('<', _) => false,
                        (_, '<') => true,
                        ('0' | 'A', '1' | '4' | '7') => true,
                        ('1' | '4' | '7', '0' | 'A') => false,
                        (_, _) => k0.x < k1.x,
                    };
                    if vertical_first {
                        press_buttons(k0.y, k1.y, 'v', '^', &mut buttons);
                        press_buttons(k0.x, k1.x, '>', '<', &mut buttons);
                    } else {
                        press_buttons(k0.x, k1.x, '>', '<', &mut buttons);
                        press_buttons(k0.y, k1.y, 'v', '^', &mut buttons);
                    }
                }
                buttons.push('A');
                e.insert(buttons)
            }
        }
    }
}

fn press_buttons(a: usize, b: usize, l: char, r: char, buttons: &mut Vec<char>) {
    if a < b {
        for _ in a..b {
            buttons.push(l);
        }
    } else {
        for _ in b..a {
            buttons.push(r);
        }
    }
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
    let mut keypad = KeyPad::new(2);
    input.lines().map(|c| keypad.enter_code(c)).sum()
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> usize {
    let mut keypad = KeyPad::new(25);
    input.lines().map(|c| keypad.enter_code(c)).sum()
}
