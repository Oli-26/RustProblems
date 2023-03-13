fn main() {
    let mut set = ConcurrentSet::new();
}

use std::sync::{Mutex, RwLock};
use std::collections::HashSet;

struct ConcurrentSet{
    set: HashSet<String>,
    lock: RwLock<()>,
}

impl ConcurrentSet{
    fn new() -> ConcurrentSet{
        ConcurrentSet{
            set: HashSet::new(),
            lock: RwLock::new(()),
        }
    }

    fn insert(&mut self, value: String){
        let _lock = self.lock.write().unwrap();
        self.set.insert(value);
    }

    fn contains(&mut self, value: &String) -> bool{
        let _lock = self.lock.read().unwrap();
        self.set.contains(value)
    }

    fn remove(&mut self, value: &String){
        let _lock = self.lock.write().unwrap();
        self.set.remove(value);
    }
}