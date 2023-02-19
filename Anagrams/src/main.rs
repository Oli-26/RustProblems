use std::io;
use std::iter::Map;
use std::collections::HashMap;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let mut input = String::new();

    println!("Enter a list of words:");
    io::stdin().read_line(&mut input).unwrap();

    let words: Vec<String> = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let mut anagram_map: HashMap<String, Vec<String>> =  create_anagram_map(words);

   

    for (_, words) in anagram_map {
        if words.len() >= 1 {
            let words_str = words.join(" ");
            println!("{}", words_str);
        }
    }
}

fn create_anagram_map(words : Vec<String>) -> HashMap<String, Vec<String>> {
    let mut anagram_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut matched_words : Vec<String> = vec![];
    for word in words.iter(){
        let mut anagrams : Vec<String> = vec![];
        if(!matched_words.contains(word)){       
            for other_word in words.iter() {
                if(!matched_words.contains(other_word) && is_anagram(word, other_word)){
                    anagrams.push(other_word.clone());
                    
                    matched_words.push(other_word.clone());
                }
            }
            anagram_map.insert(word.clone(), anagrams);
        }
    };
    anagram_map
}

fn is_anagram(chars1: &String, chars2: &String) -> bool {

    let alphabet_map : HashMap<char, u32> = alphabet_to_primes();

    fn alphabet_to_primes() -> HashMap<char, u32> {
        let prime_iter = primes(26);
        let prime_vec: Vec<u32> = prime_iter.collect();
        let mut result = HashMap::new();
        for (i, c) in ALPHABET.chars().enumerate() {
            let prime = prime_vec[i];
            result.insert(c, prime);
        }
        result
    }

    let mut product_one : u32 = 1;
    let mut product_two : u32 = 1;
    for i in chars1.chars(){
        product_one *= alphabet_map[&i];
    };

    for i in chars2.chars(){
        product_two *= alphabet_map[&i];
    };
    product_one == product_two
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f32).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn primes(n: u32) -> impl Iterator<Item = u32> {
    let mut i = 2;
    let mut count = 0;
    std::iter::from_fn(move || {
        while !is_prime(i) {
            i += 1;
        }
        let result = i;
        i += 1;
        count += 1;
        if count <= n {
            Some(result)
        } else {
            None
        }
    })
}