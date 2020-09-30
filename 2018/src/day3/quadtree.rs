use super::claim::{Claim, Rect};
use paste::paste;
use std::convert::From;

enum Content {
    Branch(Box<[Quadtree; 4]>),
    Leaf(Vec<usize>),
}

pub struct QClaim {
    pub has_conflict: bool,
    pub id: usize,
    bounds: Rect,
}

impl QClaim {
    fn overlaps(&self, other: &QClaim) -> bool {
        self.bounds.overlaps(&other.bounds)
    }
}

impl From<&Claim> for QClaim {
    fn from(claim: &Claim) -> Self {
        QClaim {
            has_conflict: false,
            id: claim.id,
            bounds: claim.bounds,
        }
    }
}

pub struct Quadtree {
    claims: Content,
    bounds: Rect,
}

macro_rules! quadrant {
    ($q:literal, $quad:ident, $all:ident, $src:ident, $x1:expr, $y1:expr, $x2:expr, $y2:expr) => {
        paste! {
            let [<q $q _rect>] = Rect::new($x1, $y1, $x2, $y2);

            let [<q $q _claims>]: Vec<usize> = $src
                .iter()
                .filter(|&&c| $all[c].bounds.overlaps(&[<q $q _rect>]))
                .map(|c| *c)
                .collect();

            let $quad = Quadtree { claims: Content::Leaf([<q $q _claims>]), bounds: [<q $q _rect>] };
        }
    };
}

impl Quadtree {
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Quadtree {
            claims: Content::Leaf(Vec::new()),
            bounds: Rect { x1, y1, x2, y2 },
        }
    }

    pub fn insert(&mut self, claim: usize, all_claims: &mut Vec<QClaim>) {
        if !self.bounds.overlaps(&all_claims[claim].bounds) {
            return;
        }

        match self.claims {
            Content::Branch(ref mut children) => {
                for child in children.iter_mut() {
                    child.insert(claim, all_claims);
                }
            }

            Content::Leaf(ref mut claims) => {
                for other in claims.iter() {
                    if all_claims[claim].overlaps(&all_claims[*other]) {
                        all_claims[claim].has_conflict = true;
                        all_claims[*other].has_conflict = true;
                    }
                }
                claims.push(claim);

                if claims.len() == 4 && self.bounds.x2 - self.bounds.x1 > 2 {
                    let rect = self.bounds;
                    let x = (rect.x1 + rect.x2) / 2;
                    let y = (rect.x1 + rect.y2) / 2;

                    quadrant!(1, q1, all_claims, claims, rect.x1, rect.y1, x, y);
                    quadrant!(2, q2, all_claims, claims, x, rect.y1, rect.x2, y);
                    quadrant!(3, q3, all_claims, claims, rect.x1, y, x, rect.y2);
                    quadrant!(4, q4, all_claims, claims, x, y, rect.x2, rect.y2);

                    self.claims = Content::Branch(Box::new([q1, q2, q3, q4]));
                }
            }
        }
    }
}
