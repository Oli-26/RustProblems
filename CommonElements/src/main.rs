use std::collections::HashSet;
use core::hash::Hash;

fn find_common_elements<T: Ord + Eq + Hash + Clone>(vec1: &Vec<T>, vec2: &Vec<T>) -> Vec<T> {
    let set1: HashSet<&T> = vec1.iter().collect();
    let set2: HashSet<&T> = vec2.iter().collect();
    let common: HashSet<_> = set1.intersection(&set2).cloned().collect();
    let mut result: Vec<T> = common.into_iter().cloned().collect();
    result.sort();
    result
}
 
fn main() {
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec![3, 4, 5, 6, 7];
    let common_elements = find_common_elements(&vec1, &vec2);
    println!("{:?}", common_elements);
}