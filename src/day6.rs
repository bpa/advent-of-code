use std::cell::{Ref, RefCell, UnsafeCell};
use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn orbitlist(input: &str) -> Vec<(String, String)> {
    input
        .split_whitespace()
        .map(|o| {
            let mut planets = o.split(")");
            (
                planets.next().unwrap().to_string(),
                planets.next().unwrap().to_string(),
            )
        })
        .collect()
}

#[aoc(day6, part1)]
fn orbits(input: &[(String, String)]) -> usize {
    let mut system = create_graph(input);
    println!("Total entries: {}", system.len());
    system
        .iter_mut()
        .map(|s| (*s.borrow_mut()).num_routes())
        .sum()
}

fn create_graph(input: &[(String, String)]) -> Vec<RefCell<Node>> {
    let system: UnsafeCell<Vec<RefCell<Node>>> = UnsafeCell::new(Vec::new());
    let mut lookup: HashMap<&str, usize> = HashMap::new();

    macro_rules! find {
        ( $name:expr ) => {{
            *lookup.entry($name).or_insert_with(|| unsafe {
                let s: *mut Vec<RefCell<Node>> = system.get();
                let id: usize = (*s).len();
                (*s).push(RefCell::new(Node::new($name, id)));
                id
            })
        }};
    }

    for orbit in input {
        let satellite = find!(&orbit.1);
        let planet = find!(&orbit.0);

        unsafe {
            let sys: *mut Vec<RefCell<Node>> = system.get();
            let sat: &RefCell<Node> = &(*sys)[satellite];
            let center: Ref<Node> = ((*sys)[planet]).borrow();
            let satellites: *mut Vec<&RefCell<Node>> = center.satellites.get();
            print!("{}|{}:", center.pos, center.name);
            for s in (*satellites).iter() {
                let b: Ref<Node> = s.borrow();
                print!(" {}|{}", b.pos, b.name);
            }
            println!();
            (*satellites).push(sat);

            print!("{}|{}:", center.pos, center.name);
            for s in (*satellites).iter() {
                print!(" {}|{}", s.borrow().pos, s.borrow().name);
            }
            println!();
        }
    }

    system.into_inner()
}

#[derive(Debug)]
enum Visited {
    No,
    InProgress,
    Yes,
}

struct Node<'a> {
    name: &'a str,
    pos: usize,
    satellites: UnsafeCell<Vec<&'a RefCell<Node<'a>>>>,
    visited: Visited,
    routes: usize,
}

impl<'a> Node<'a> {
    fn new(name: &'a str, pos: usize) -> Self {
        Node {
            name: name,
            pos: pos,
            satellites: UnsafeCell::new(Vec::new()),
            visited: Visited::No,
            routes: 0,
        }
    }

    fn num_routes(&mut self) -> usize {
        println!("1");
        println!("{} {}: {:?}", self.pos, self.name, self.visited);
        match self.visited {
            Visited::Yes => {
                println!("yes");
                self.routes
            }
            Visited::InProgress => {
                println!("Cycle in graph detected");
                self.visited = Visited::Yes;
                0
            }
            Visited::No => {
                println!("no");
                print!("{}:", self.name);
                unsafe {
                    for s in (*self.satellites.get()).iter() {
                        let sat = s.borrow();
                        print!(" {}|{}", sat.pos, sat.name);
                    }
                }
                println!();
                self.visited = Visited::InProgress;
                let sat = unsafe { &mut *self.satellites.get() };
                self.routes = sat.len()
                    + sat
                        .iter_mut()
                        .map(|s| (*s.borrow_mut()).num_routes())
                        .sum::<usize>();
                self.visited = Visited::Yes;
                println!("Done!");
                self.routes
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::{Display, Formatter, Result};

    impl<'a> Display for Node<'a> {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "{} => {:?}", self.name, unsafe {
                (*self.satellites.get())
                    .iter()
                    .map(|s| s.borrow().name.clone())
                    .collect::<Vec<&str>>()
            })
        }
    }

    macro_rules! s {
        ( $str:ident ) => {
            String::from(stringify!($str))
        };
    }
    #[test]
    fn single_link() {
        let input = vec![(s!(a), s!(b)), (s!(b), s!(c)), (s!(b), s!(d))];
        let mut graph = create_graph(&input);
        for node in graph.iter() {
            println!("{}", node.borrow());
        }
        assert_eq!(
            4usize,
            graph
                .iter_mut()
                .map(|s| (*s.borrow_mut()).num_routes())
                .sum()
        );
    }
}
