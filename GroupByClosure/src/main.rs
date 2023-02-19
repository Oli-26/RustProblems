use std::{collections::HashMap, hash::Hash};

fn group_by<T, F>(vec: Vec<T>, f: F) -> HashMap<i32, Vec<T>>
where
    F: Fn(&T) -> i32,
{
    let mut map = HashMap::new();
    for element in vec{
        let new_vec : &mut Vec<T> = map.entry(f(&element)).or_insert_with(|| vec![]);
        new_vec.push(element);
    }
    map
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let f = |x: &i32| { x % 3 };
    let result = group_by(vec, f);
    println!("{:?}", result);
}