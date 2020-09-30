mod claim;
mod quadtree;

use self::claim::Claim;
use self::quadtree::{QClaim, Quadtree};

#[aoc_generator(day3)]
fn parse_claims(input: &str) -> Vec<Claim> {
    input.lines().map(|claim| Claim::from(claim)).collect()
}

#[aoc(day3, part1)]
fn multi_claims(claims: &Vec<Claim>) -> usize {
    let mut whole_fabric = vec![0usize; 1000 * 1000];
    let fabric = whole_fabric.as_mut_slice();
    let mut overlapping = 0;
    for claim in claims {
        for x in claim.bounds.x1..claim.bounds.x2 {
            for y in claim.bounds.y1..claim.bounds.y2 {
                let i = x + y * 1000;
                fabric[i] += 1;
                if fabric[i] == 2 {
                    overlapping += 1;
                }
            }
        }
    }
    overlapping
}

#[aoc(day3, part2)]
fn the_one_good_claim(claims: &[Claim]) -> usize {
    'outer: for i in 0..claims.len() {
        let claim = &claims[i];
        for j in 0..claims.len() {
            let other = &claims[j];
            if claim.overlaps(other) && claim.id != other.id {
                continue 'outer;
            }
        }
        return claim.id;
    }
    0
}

#[aoc(day3, part2, alt1)]
fn the_one_quadtree(input: &Vec<Claim>) -> usize {
    let mut claims: Vec<QClaim> = input.iter().map(|c| QClaim::from(c)).collect();
    let mut tree = Quadtree::new(0, 0, 1000, 1000);
    for claim in 0..input.len() {
        tree.insert(claim, &mut claims);
    }
    claims.iter().find(|claim| !claim.has_conflict).unwrap().id
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::day3::claim::Rect;

    const CLAIMS: &str = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";

    #[test]
    fn parse() {
        assert_eq!(
            vec![
                Claim {
                    id: 1,
                    bounds: Rect {
                        x1: 1,
                        y1: 3,
                        x2: 5,
                        y2: 7
                    }
                },
                Claim {
                    id: 2,
                    bounds: Rect {
                        x1: 3,
                        y1: 1,
                        x2: 7,
                        y2: 5
                    }
                },
                Claim {
                    id: 3,
                    bounds: Rect {
                        x1: 5,
                        y1: 5,
                        x2: 7,
                        y2: 7
                    }
                }
            ],
            parse_claims(CLAIMS)
        )
    }

    #[test]
    fn part1() {
        assert_eq!(4, multi_claims(&parse_claims(CLAIMS)))
    }

    #[test]
    fn part2() {
        assert_eq!(3, the_one_good_claim(&parse_claims(CLAIMS)))
    }

    #[test]
    fn part2_quadtree() {
        assert_eq!(3, the_one_quadtree(&parse_claims(CLAIMS)))
    }
}
