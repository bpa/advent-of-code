use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[aoc(day23, part1)]
pub fn part1(input: &str) -> usize {
    let mut pairs = HashMap::new();
    let mut seen = HashSet::new();
    let mut want: HashMap<(&str, &str), Vec<&str>> = HashMap::new();
    let mut found = 0;
    for link in input.lines() {
        let mut p = link.split("-");
        let mut a = p.next().unwrap();
        let mut b = p.next().unwrap();
        if a > b {
            (a, b) = (b, a);
        }

        seen.insert((a, b));
        if let Some(q) = want.get(&(a, b)) {
            let ok = a.starts_with("t") || b.starts_with("t");
            for c in q.iter() {
                if ok || c.starts_with("t") {
                    found += 1;
                }
            }
        }

        let cluster: &mut Vec<&str> = pairs.entry(a).or_insert_with(|| Vec::new());
        for &l in cluster.iter() {
            let key = if b < l { (b, l) } else { (l, b) };
            if seen.contains(&key) {
                if a.starts_with("t") || b.starts_with("t") || l.starts_with("t") {
                    found += 1;
                }
            } else {
                want.entry(key).or_insert_with(|| Vec::new()).push(a);
            }
        }
        cluster.push(b);
    }
    found
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> String {
    let mut pairs = HashMap::new();
    for link in input.lines() {
        let mut p = link.split("-");
        let mut a = p.next().unwrap();
        let mut b = p.next().unwrap();
        if a > b {
            (a, b) = (b, a);
        }

        pairs.entry(a).or_insert_with(|| Vec::new()).push(b);
    }
    for pair in pairs.iter_mut() {
        pair.1.sort();
    }
    let mut lan = Vec::new();
    for (a, links) in pairs.iter() {
        if links.len() < lan.len() {
            continue;
        }
        let mut cluster = fully_connected(&links, &links, &pairs);
        cluster.push(*a);
        if cluster.len() > lan.len() {
            lan = cluster;
        }
    }
    lan.reverse();
    lan.join(",")
}

fn fully_connected<'a>(
    links: &Vec<&'a str>,
    joined: &Vec<&'a str>,
    pairs: &HashMap<&'a str, Vec<&'a str>>,
) -> Vec<&'a str> {
    let lan = intersection(links, joined);

    if lan.len() == 1 {
        return lan;
    }

    let mut best = Vec::new();
    for &link in lan.iter() {
        if best.len() < link.len() {
            if let Some(cluster) = pairs.get(link) {
                let mut next = fully_connected(cluster, &lan, pairs);
                next.push(link);
                if next.len() > best.len() {
                    best = next;
                }
            }
        }
    }
    best
}

fn intersection<'a>(a: &Vec<&'a str>, b: &Vec<&'a str>) -> Vec<&'a str> {
    let mut c = Vec::with_capacity(32);
    let mut left = a.iter();
    let mut right = b.iter();
    let mut x = *left.next().unwrap();
    let mut y = *right.next().unwrap();
    loop {
        match x.cmp(y) {
            Ordering::Less => match left.next() {
                Some(&v) => x = v,
                None => break,
            },
            Ordering::Equal => {
                c.push(x);
                match left.next() {
                    Some(&v) => x = v,
                    None => break,
                }
                match right.next() {
                    Some(&v) => y = v,
                    None => break,
                }
            }
            Ordering::Greater => match right.next() {
                Some(&v) => y = v,
                None => break,
            },
        }
    }
    c
}
