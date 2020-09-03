use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Register {
    map: HashMap<String, usize>,
    next: usize,
}

impl Register {
    pub fn new() -> Self {
        Register {
            map: HashMap::new(),
            next: 0,
        }
    }

    pub fn get(&mut self, register: &str) -> usize {
        match self.map.entry(String::from(register)) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                e.insert(self.next);
                self.next += 1;
                self.next - 1
            }
        }
    }

    pub fn len(&self) -> usize {
        self.next
    }
}
