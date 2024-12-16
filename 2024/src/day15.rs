use aoc::*;

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let warehouse = Grid::from(input.split("\n\n").next().unwrap());
    let mut bot = warehouse.index_of('@').unwrap();
    for mv in input.chars() {
        bot = match mv {
            '^' => move_bot(bot, 0, -1),
            '>' => move_bot(bot, 1, 0),
            'v' => move_bot(bot, 0, 1),
            '<' => move_bot(bot, -1, 0),
            _ => bot,
        }
    }
    warehouse
        .points()
        .filter(|p| p.get() == 'O')
        .map(|p| p.y * 100 + p.x)
        .sum()
}

fn move_bot(bot: Point2D<char>, x: isize, y: isize) -> Point2D<char> {
    match swap(bot.clone(), x, y) {
        Some(next) => {
            bot.set('.');
            next
        }
        None => bot,
    }
}

fn swap(a: Point2D<char>, x: isize, y: isize) -> Option<Point2D<char>> {
    let next = a.neighbor(x, y)?;
    match next.get() {
        '.' => {
            next.set(a.get());
            Some(next)
        }
        'O' => match swap(next.clone(), x, y) {
            Some(_) => {
                next.set(a.get());
                Some(next)
            }
            None => None,
        },
        _ => None,
    }
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let old_warehouse = Grid::from(input.split("\n\n").next().unwrap());
    let mut warehouse = Grid::of(old_warehouse.width() * 2, old_warehouse.height(), '.');
    for p in old_warehouse.points() {
        match p.get() {
            '#' => {
                warehouse.set(p.x * 2, p.y, '#');
                warehouse.set(p.x * 2 + 1, p.y, '#');
            }
            'O' => {
                warehouse.set(p.x * 2, p.y, '[');
                warehouse.set(p.x * 2 + 1, p.y, ']');
            }
            '@' => warehouse.set(p.x * 2, p.y, '@'),
            _ => {}
        }
    }
    let mut bot = warehouse.index_of('@').unwrap();
    for mv in input.chars() {
        bot = match mv {
            '^' => push_vertical(&mut warehouse, bot.x, bot.y, -1),
            '>' => push_horizontal(&mut warehouse, bot.x, bot.y, 1),
            'v' => push_vertical(&mut warehouse, bot.x, bot.y, 1),
            '<' => push_horizontal(&mut warehouse, bot.x, bot.y, -1),
            _ => bot,
        };
    }
    warehouse
        .points()
        .filter(|p| p.get() == '[')
        .map(|p| p.y * 100 + p.x)
        .sum()
}

fn push_horizontal(warehouse: &mut Grid<char>, x: usize, y: usize, dx: isize) -> Point2D<char> {
    let mut nx = x;
    loop {
        nx = (nx as isize + dx) as usize;
        match warehouse.get(nx, y).unwrap() {
            '.' => break,
            '#' => return warehouse.at(x, y).unwrap(),
            _ => {}
        }
    }
    nx = x;
    let mut value = warehouse.get(x, y).unwrap();
    warehouse.set(x, y, '.');
    while value != '.' {
        nx = (nx as isize + dx) as usize;
        let next = warehouse.get(nx, y).unwrap();
        warehouse.set(nx, y, value);
        value = next;
    }
    warehouse.at((x as isize + dx) as usize, y).unwrap()
}

fn push_vertical(warehouse: &mut Grid<char>, x: usize, y: usize, dy: isize) -> Point2D<char> {
    if can_push(warehouse, x, y as isize, dy) {
        push_vertical_unchecked(warehouse, x, y as isize, dy);
        return warehouse.at(x, (y as isize + dy) as usize).unwrap();
    } else {
        return warehouse.at(x, y).unwrap();
    }
}

fn can_push(warehouse: &mut Grid<char>, x: usize, mut y: isize, dy: isize) -> bool {
    loop {
        y += dy;
        match warehouse.get(x, y as usize).unwrap() {
            '#' => return false,
            '[' => {
                if !can_push(warehouse, x + 1, y, dy) {
                    return false;
                }
            }
            ']' => {
                if !can_push(warehouse, x - 1, y, dy) {
                    return false;
                }
            }
            '.' => return true,
            _ => {}
        }
    }
}

fn push_vertical_unchecked(warehouse: &mut Grid<char>, x: usize, mut y: isize, dy: isize) {
    let mut val = warehouse.get(x, y as usize).unwrap();
    warehouse.set(x, y as usize, '.');
    y += dy;
    let mut next = warehouse.get(x, y as usize).unwrap();
    warehouse.set(x, y as usize, val);
    loop {
        match next {
            '.' => break,
            '[' => push_vertical_unchecked(warehouse, x + 1, y, dy),
            ']' => push_vertical_unchecked(warehouse, x - 1, y, dy),
            _ => {}
        }
        y += dy;
        val = warehouse.get(x, y as usize).unwrap();
        warehouse.set(x, y as usize, next);
        next = val;
    }
}
