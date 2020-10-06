use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_till};
use nom::character::complete::digit1;
use nom::combinator::{map, value};
use nom::sequence::pair;
use nom::IResult;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum EventType {
    StartShift(usize),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, PartialEq)]
struct Event {
    day: usize,
    hour: usize,
    minute: usize,
    event_type: EventType,
}

fn parse_entry(input: &str) -> IResult<&str, Event> {
    let (input, _) = take_till(|c| c == '[')(input)?;
    let (input, _) = take(9usize)(input)?;
    let (input, day) = map(take(2usize), |n: &str| n.parse::<usize>().unwrap())(input)?;
    let (input, _) = take(1usize)(input)?;
    let (input, hour) = map(take(2usize), |n: &str| n.parse::<usize>().unwrap())(input)?;
    let (input, _) = take(1usize)(input)?;
    let (input, minute) = map(take(2usize), |n: &str| n.parse::<usize>().unwrap())(input)?;
    let (input, _) = take(2usize)(input)?;
    let (input, event_type) = alt((
        map(pair(tag("Guard #"), digit1), |(_, id): (&str, &str)| {
            EventType::StartShift(id.parse::<usize>().unwrap())
        }),
        value(EventType::FallsAsleep, tag("falls")),
        value(EventType::WakesUp, tag("wakes")),
    ))(input)?;
    Ok((
        input,
        Event {
            day,
            hour,
            minute,
            event_type,
        },
    ))
}

#[aoc_generator(day4)]
fn parse_log(input: &str) -> Vec<(usize, [usize; 60])> {
    let mut log: Vec<&str> = input.lines().collect();
    log.sort();
    let mut guards = HashMap::new();
    let mut guard: Option<&mut [usize; 60]> = None;
    let mut start_snooze = 0;
    for event in log.iter().map(|l| parse_entry(l).unwrap().1) {
        match event.event_type {
            EventType::StartShift(id) => guard = Some(guards.entry(id).or_insert_with(|| [0; 60])),
            EventType::FallsAsleep => start_snooze = event.minute,
            EventType::WakesUp => {
                if let Some(ref mut freq) = guard {
                    for m in start_snooze..event.minute {
                        freq[m] += 1;
                    }
                }
            }
        }
    }
    guards.into_iter().collect()
}

#[aoc(day4, part1)]
fn snooziest_guard(guards: &Vec<(usize, [usize; 60])>) -> usize {
    guards
        .iter()
        .map(|(id, freq): &(usize, [usize; 60])| (id, freq.iter().sum::<usize>(), freq))
        .max_by_key(|(_, total, _)| *total)
        .map(|(&id, _, freq)| (id, freq.iter().enumerate().max_by_key(|(_, v)| *v).unwrap()))
        .map(|(id, (minute, _))| id * minute)
        .unwrap()
}

#[aoc(day4, part2)]
fn snooziest_minute(guards: &Vec<(usize, [usize; 60])>) -> usize {
    guards
        .iter()
        .map(|(id, freq)| (id, freq.iter().enumerate().max_by_key(|(_, &v)| v).unwrap()))
        .max_by_key(|(_, (_, &freq))| freq)
        .map(|(&id, (minute, _))| id * minute)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const OBSERVATIONS: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up";

    #[test]
    fn test_parsing() {
        let mut hour = [0; 60];
        for minute in 5..25 {
            hour[minute] = 1;
        }

        assert_eq!(vec![(10, hour)], parse_log(OBSERVATIONS));
    }
}
