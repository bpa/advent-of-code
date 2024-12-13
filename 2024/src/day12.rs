use aoc::*;

type Plot = Point2D<(u8, bool)>;

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    Grid::parse_str(input, |p| (p, false))
        .points()
        .map(fence)
        .sum()
}

fn fence(p: Plot) -> usize {
    let plant_type = {
        let s = p.get();
        if s.1 {
            return 0;
        }
        s.0
    };
    let mut area = 0;
    let mut perimeter = 0;
    let mut queue = Vec::new();
    queue.push(p);
    while let Some(next) = queue.pop() {
        if next.get().1 {
            continue;
        }
        area += 1;
        perimeter += 4;
        next.set((plant_type, true));
        for n in next.adjacent() {
            let nv = n.get();
            if nv.0 == plant_type {
                if nv.1 {
                    perimeter -= 2;
                } else {
                    queue.push(n);
                }
            }
        }
    }
    area * perimeter
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    Grid::parse_str(input, |p| (p, false))
        .points()
        .map(discount_fence)
        .sum()
}

fn discount_fence(p: Plot) -> usize {
    let plant_type = {
        let s = p.get();
        if s.1 {
            return 0;
        }
        s.0
    };
    let mut area = 0;
    let mut perimeter = 0;
    let mut queue = Vec::new();
    queue.push(p.clone());
    while let Some(next) = queue.pop() {
        if next.get().1 {
            continue;
        }
        area += 1;
        next.set((plant_type, true));
        let surrounding = get_surrounding(&next);
        perimeter += turns(&surrounding, 6, 7, 0, 1, 2);
        perimeter += turns(&surrounding, 0, 1, 2, 3, 4);
        perimeter += turns(&surrounding, 2, 3, 4, 5, 6);
        perimeter += turns(&surrounding, 4, 5, 6, 7, 0);
        if surrounding[0] == Status::Full {
            queue.push(next.n().unwrap());
        }
        if surrounding[2] == Status::Full {
            queue.push(next.e().unwrap());
        }
        if surrounding[4] == Status::Full {
            queue.push(next.s().unwrap());
        }
        if surrounding[6] == Status::Full {
            queue.push(next.w().unwrap());
        }
    }
    (area * perimeter) as usize
}

fn turns(s: &[Status; 8], w: usize, nw: usize, n: usize, ne: usize, e: usize) -> isize {
    if s[n] != Status::Empty {
        return 0;
    }
    if s[w] == Status::Visited
        && s[e] == Status::Visited
        && s[nw] == Status::Empty
        && s[ne] == Status::Empty
    {
        return -1;
    }
    if s[n] != Status::Empty
        || (s[w] == Status::Visited && s[nw] == Status::Empty)
        || (s[e] == Status::Visited && s[ne] == Status::Empty)
    {
        return 0;
    }
    1
}

#[derive(Copy, Clone, PartialEq)]
enum Status {
    Visited,
    Empty,
    Full,
}

fn get_surrounding(p: &Plot) -> [Status; 8] {
    let plant = p.get().0;
    let keep = |p: Option<Plot>| match p {
        Some(n) => {
            let plot = n.get();
            match plot.0 == plant {
                true => match plot.1 {
                    true => Status::Visited,
                    false => Status::Full,
                },
                false => Status::Empty,
            }
        }
        None => Status::Empty,
    };

    [
        keep(p.n()),
        keep(p.ne()),
        keep(p.e()),
        keep(p.se()),
        keep(p.s()),
        keep(p.sw()),
        keep(p.w()),
        keep(p.nw()),
    ]
}
