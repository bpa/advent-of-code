use std::{iter::Peekable, ops::Add, str::Chars};

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<SnailNumber> {
    input.lines().map(SnailNumber::from).collect()
}

#[aoc(day18, part1)]
fn part1(input: &[SnailNumber]) -> usize {
    let mut input = input.iter();
    let mut a = input.next().unwrap().clone();
    for b in input {
        a = a + b;
    }
    a.magnitude()
}

#[aoc(day18, part2)]
fn part2(input: &[SnailNumber]) -> usize {
    let mut max = 0;
    for (i, a) in input.iter().enumerate() {
        for (j, b) in input.iter().enumerate() {
            if i != j {
                let magnitude = (a.clone() + b).magnitude();
                if magnitude > max {
                    max = magnitude;
                }
            }
        }
    }
    max
}

#[derive(Clone, PartialEq)]
enum SnailNumber {
    Leaf(usize),
    Node(Box<SnailNumber>, Box<SnailNumber>),
}

impl std::fmt::Debug for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Leaf(arg0) => f.write_fmt(format_args!("{}", arg0)),
            Self::Node(arg0, arg1) => f.debug_list().entries(vec![arg0, arg1]).finish(),
        }
    }
}

fn parse_num(chars: &mut Peekable<Chars>) -> SnailNumber {
    let mut values = Vec::new();
    loop {
        let c = chars.next().unwrap();
        match c {
            '[' => values.push(parse_num(chars)),
            ',' | ' ' => {}
            ']' => {
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                return SnailNumber::Node(Box::new(a), Box::new(b));
            }
            n => {
                let mut v = n.to_digit(10).unwrap() as usize;
                while chars.peek().unwrap().is_digit(10) {
                    v *= 10;
                    v += chars.next().unwrap().to_digit(10).unwrap() as usize;
                }
                values.push(SnailNumber::Leaf(v))
            }
        }
    }
}

impl From<&str> for SnailNumber {
    fn from(input: &str) -> Self {
        let mut chars = input.chars().peekable();
        chars.next();
        parse_num(&mut chars)
    }
}

impl Add<&SnailNumber> for SnailNumber {
    type Output = SnailNumber;

    fn add(self, rhs: &SnailNumber) -> Self::Output {
        let mut new = SnailNumber::Node(Box::new(self.clone()), Box::new(rhs.clone()));
        loop {
            if new.explode(0).is_some() {
                continue;
            }
            if new.split() {
                continue;
            }
            break;
        }
        new
    }
}

impl SnailNumber {
    fn split(&mut self) -> bool {
        if let SnailNumber::Node(a, b) = self {
            for child in [a, b] {
                if let SnailNumber::Leaf(n) = **child {
                    if n > 9 {
                        let a = n / 2;
                        let b = n - a;
                        *child = Box::new(SnailNumber::Node(
                            Box::new(SnailNumber::Leaf(a)),
                            Box::new(SnailNumber::Leaf(b)),
                        ));
                        return true;
                    }
                } else {
                    if child.split() {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn explode(&mut self, depth: usize) -> Option<(usize, usize)> {
        if let SnailNumber::Node(ref mut a, ref mut b) = self {
            if depth == 4 {
                if let SnailNumber::Leaf(l) = **a {
                    if let SnailNumber::Leaf(r) = **b {
                        return Some((l, r));
                    }
                }
                println!("{:?}", self);
                unreachable!();
            }

            if let Some((l, r)) = a.explode(depth + 1) {
                if depth == 3 {
                    **a = SnailNumber::Leaf(0);
                }
                let mut parent = b;
                while let SnailNumber::Node(ref mut child, _) = **parent {
                    parent = child;
                }
                if let SnailNumber::Leaf(v) = **parent {
                    **parent = SnailNumber::Leaf(v + r);
                }
                return Some((l, 0));
            }

            if let Some((l, r)) = b.explode(depth + 1) {
                if depth == 3 {
                    **b = SnailNumber::Leaf(0);
                }
                let mut parent = a;
                while let SnailNumber::Node(_, ref mut child) = **parent {
                    parent = child;
                }
                if let SnailNumber::Leaf(v) = **parent {
                    **parent = SnailNumber::Leaf(l + v);
                }
                return Some((0, r));
            }
        }
        None
    }

    fn magnitude(&self) -> usize {
        match self {
            SnailNumber::Node(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
            SnailNumber::Leaf(v) => *v,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! v {
        ($v:literal) => {
            SnailNumber::Leaf($v)
        };
    }

    macro_rules! snail {
        ($a:expr, $b:expr) => {
            SnailNumber::Node(Box::new($a), Box::new($b))
        };
    }

    #[test]
    fn test_parsing() {
        assert_eq!(SnailNumber::from("[1,2]"), snail![v!(1), v!(2)]);
        assert_eq!(
            SnailNumber::from("[[1,2],3]"),
            snail![snail![v!(1), v!(2)], v!(3)]
        );
    }

    #[test]
    fn test_splitting() {
        for (input, expected) in [
            ("[10, 1]", "[[5,5], 1]"),
            ("[1, [11, 12]]", "[1, [[5, 6], 12]]"),
            ("[2, [1, [3, 12]]]", "[2, [1, [3, [6, 6]]]]"),
        ] {
            let mut a = SnailNumber::from(input);
            a.split();
            assert_eq!(a, SnailNumber::from(expected));
        }
    }

    #[test]
    fn test_explode() {
        for (input, expected) in [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ] {
            let mut a = SnailNumber::from(input);
            a.explode(0);
            assert_eq!(a, SnailNumber::from(expected));
        }
    }

    #[test]
    fn test_add() {
        let a = SnailNumber::from("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = SnailNumber::from("[1,1]");
        assert_eq!(
            &a + &b,
            SnailNumber::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }
}
