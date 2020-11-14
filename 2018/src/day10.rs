use crate::number;
use nom::character::is_digit;
use nom::{bytes::complete::take_till, IResult};
use std::cmp::{max, min};

struct Star {
    x: isize,
    y: isize,
    x_velocity: isize,
    y_velocity: isize,
}

//position=< -9908,  -9884> velocity=< 1,  1>
fn parse_star(input: &str) -> IResult<&str, Star> {
    let (input, _) = take_till(|c| c == '-' || is_digit(c as u8))(input)?;
    let (input, x) = number(input).unwrap();
    let (input, _) = take_till(|c| c == '-' || is_digit(c as u8))(input)?;
    let (input, y) = number(input).unwrap();
    let (input, _) = take_till(|c| c == '-' || is_digit(c as u8))(input)?;
    let (input, x_velocity) = number(input).unwrap();
    let (input, _) = take_till(|c| c == '-' || is_digit(c as u8))(input)?;
    let (input, y_velocity) = number(input).unwrap();
    Ok((
        input,
        Star {
            x,
            y,
            x_velocity,
            y_velocity,
        },
    ))
}

#[aoc_generator(day10)]
fn parse_stars(input: &str) -> Vec<Star> {
    input
        .lines()
        .map(|line| parse_star(line).unwrap().1)
        .collect()
}

#[aoc(day10, part1)]
fn find_message(stars: &Vec<Star>) -> String {
    let ticks = how_long_do_we_wait(stars);

    let mut message: Vec<(isize, isize)> = stars
        .iter()
        .map(|s| (s.y + s.y_velocity * ticks, s.x + s.x_velocity * ticks))
        .collect();

    message.sort();
    let col_0 = *message.iter().map(|(_, x)| x).min().unwrap();

    let mut raw_str = Vec::new();
    let mut column = 0;
    let mut row = isize::MIN;

    for (y, x) in message {
        if y != row {
            column = col_0;
            row = y;
            raw_str.push(b'\n');
        }
        if x >= column {
            for _ in column..x {
                raw_str.push(b' ');
            }
            raw_str.push(b'*');
            column = x + 1;
        }
    }
    String::from_utf8(raw_str).unwrap()
}

#[aoc(day10, part2)]
fn how_long_do_we_wait(stars: &Vec<Star>) -> isize {
    let mut width = isize::MAX;
    let mut ticks = 0;
    loop {
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        for star in stars {
            let x = star.x + star.x_velocity * ticks;
            min_x = min(min_x, x);
            max_x = max(max_x, x);
        }
        if width < max_x - min_x {
            // The stars are diverging, the last tick was our winner
            ticks -= 1;
            break;
        } else {
            width = max_x - min_x;
            ticks += 1;
        }
    }
    ticks
}
