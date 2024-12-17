mod tests;

use rand::Rng;
use std::collections::HashMap;

struct RandomizedSet {
    vals: Vec<i32>,
    cache: HashMap<i32, usize>,
    index: usize,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet{ vals: vec![0; 100001], cache: HashMap::new(), index: 0 }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.cache.contains_key(&val) { return false; }
        self.cache.insert(val, self.index);
        self.vals[self.index] = val;
        self.index += 1;
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.cache.contains_key(&val) { return false; }
        let pos = self.cache.remove(&val).unwrap();
        self.vals[pos] = self.vals[self.index - 1];
        if self.cache.contains_key(&self.vals[self.index - 1]) {
            self.cache.insert(self.vals[self.index - 1], pos);
        }
        self.index -= 1;
        true
    }

    fn get_random(&self) -> i32 {
        self.vals[rand::thread_rng().gen_range(0, self.index)]
    }

}

fn main() {
    println!("Hello, world!");
}
