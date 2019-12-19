use std::boxed::Box;
use std::cell::UnsafeCell;
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
    system.iter_mut().map(|s| s.num_routes()).sum()
}

fn create_graph(input: &[(String, String)]) -> Vec<Box<Node>> {
    let system: UnsafeCell<Vec<Box<Node>>> = UnsafeCell::new(Vec::new());
    let mut lookup: HashMap<&str, usize> = HashMap::new();

    macro_rules! find {
        ( $name:expr ) => {{
            *lookup.entry($name).or_insert_with(|| unsafe {
                let s: *mut Vec<Box<Node>> = system.get();
                let id: usize = (*s).len();
                (*s).push(Box::new(Node::new()));
                id
            })
        }};
    }

    for orbit in input {
        let satellite = find!(&orbit.1);
        let planet = find!(&orbit.0);

        unsafe {
            let sys: *mut Vec<Box<Node>> = system.get();
            let sat: &mut Box<Node> = &mut (*sys)[satellite];
            let center: &mut Box<Node> = &mut (*sys)[planet];
            let node: &mut Node = &mut *center;
            let satellites: *mut Vec<&mut Node> = (*node).satellites.get();
            (*satellites).push(&mut *sat);
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
    satellites: UnsafeCell<Vec<&'a mut Node<'a>>>,
    visited: Visited,
    routes: usize,
}

impl<'a> Node<'a> {
    fn new() -> Self {
        Node {
            satellites: UnsafeCell::new(Vec::new()),
            visited: Visited::No,
            routes: 0,
        }
    }

    fn num_routes(&mut self) -> usize {
        match self.visited {
            Visited::Yes => self.routes,
            Visited::InProgress => {
                panic!("Cycle in graph detected");
            }
            Visited::No => {
                self.visited = Visited::InProgress;
                let sat = unsafe { &mut *self.satellites.get() };
                self.routes = sat.len() + sat.iter_mut().map(|s| s.num_routes()).sum::<usize>();
                self.visited = Visited::Yes;
                self.routes
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! s {
        ( $str:ident ) => {
            String::from(stringify!($str))
        };
    }
    #[test]
    fn single_link() {
        let input = vec![(s!(a), s!(b)), (s!(b), s!(c)), (s!(b), s!(d))];
        let mut graph = create_graph(&input);
        assert_eq!(2usize, graph[0].num_routes()); //b
        assert_eq!(3usize, graph[1].num_routes()); //a
        assert_eq!(0usize, graph[2].num_routes()); //c
        assert_eq!(0usize, graph[3].num_routes()); //d
    }
}
