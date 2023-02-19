use std::collections::HashMap;

fn group_by_first_char(strings: Vec<String>) -> HashMap<char, Vec<String>> {
    let mut map: HashMap<char, Vec<String>> = HashMap::new();
    let mut group_by_first_char = | str_to_map : String | {
        let character : char = str_to_map.chars().next().unwrap();
        let mut new_vec : &mut Vec<String> = map.entry(character).or_insert_with(|| vec![]);
        new_vec.push(str_to_map);
    };

    for str in strings {
        group_by_first_char(str);
    }
    map
}


fn main() {
    let strings = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "apricot".to_string(),
        "blueberry".to_string(),
        "date".to_string(),
    ];
    let map = group_by_first_char(strings);
    println!("{:?}", map);
}
