#[derive(Debug, Default, Clone)]
struct RadixNode {
    children: Vec<RadixNode>,
    value: Option<String>,
}

impl RadixNode {
    fn insert(&mut self, value_to_add: String) {
        let mut inserted_to_node = false;
        let val = &self.value.clone();
        match val {
            Some (prefix) => {
                let mut longest_matching_prefix: String = String::new(); 
                let mut to_match = value_to_add.chars();
                for c in prefix.chars() {
                    match to_match.next() {
                        Some ( match_c ) => {
                            if c == match_c {
                                longest_matching_prefix.push(c);
                            }else{
                                break;
                            }
                        },
                        _ => break
                    }
                };

                if longest_matching_prefix == *prefix {
                    let left_over_for_insert_value = value_to_add[longest_matching_prefix.len()..].to_string();
                    let mut any_matches = false;
                    for child in self.children.iter_mut(){
                        match child.value.as_ref(){
                            Some( child_prefix ) => {
                               if child_prefix.chars().next() == left_over_for_insert_value.chars().next(){
                                    any_matches = true;
                                    inserted_to_node = true;
                                    child.insert(left_over_for_insert_value.to_string());
                                    
                                    break;
                               } 
                            },
                            None => {}
                        }
                    }

                    if !any_matches{
                        inserted_to_node = true;
                        self.children.push(RadixNode { children: vec![], value: Some(left_over_for_insert_value.to_string())})
                    }
                }else if longest_matching_prefix.len() > 0{                    
                    self.value = Some(longest_matching_prefix.to_string());
                    let left_over_due_to_new_prefix = prefix[longest_matching_prefix.len()..].to_string();
                    let left_over_for_insert_value = value_to_add[longest_matching_prefix.len()..].to_string();
                    
                    let modified_node = RadixNode { children: self.children.clone(), value: Some(left_over_due_to_new_prefix.clone()) };
                    self.children = vec![];
                    let new_node: RadixNode = RadixNode { children: vec![], value: Some(left_over_for_insert_value.clone())};
                    self.children.push(modified_node);
                    self.children.push(new_node);
                    inserted_to_node = true;
                }else{
                    self.value = None;
                    let modified_node = RadixNode { children: self.children.clone(), value: Some(prefix.to_string()) };
                    self.children = vec![];
                    let new_node: RadixNode = RadixNode { children: vec![], value: Some(value_to_add.to_string())};
                    self.children.push(modified_node);
                    self.children.push(new_node);
                    inserted_to_node = true;
                }
            },
            None => {
                for child in self.children.iter_mut(){
                    match child.value.clone(){
                        Some( child_prefix ) => {
                           if child_prefix.chars().next() == value_to_add.chars().next(){
                                child.insert(value_to_add.to_string());
                                inserted_to_node = true;
                                break;
                           } 
                        },
                        None => {}
                    }
                }     
            }
        }
        if !inserted_to_node{
            self.children.push(RadixNode { children: vec![], value: Some(value_to_add.clone())});
        }
    }

    fn find(&self, query: &str) -> Vec<String> {
        let mut return_vector: Vec<String> = vec![];

        if query.len() == 0{
            if self.children.len() == 0{
                return vec![self.value.clone().unwrap()];
            }else{
                for child in self.children.clone() {
                    let matching_strings: Vec<String> = child.find("");
                    match &self.value {
                        Some ( val ) => {
                            for matching in matching_strings {
                                let mut new_string = val.clone();
                                (new_string).push_str(&matching);
                                return_vector.push(new_string);
                            }
                        },
                        None => {
                            for matching in matching_strings {
                                return_vector.push(matching);
                            }
                        }
                    }
                }
                return return_vector;

            }
        }

        for child in self.children.clone() {
            match &child.value {
                Some ( str) => {
                    println!("looping through children: child = {}", str);
                        if query.len() >= str.len() && str[..] == query[..str.len()]{
                        println!("prefix match found! Searching nodes for {}", &query[str.len()..]);
                        
                        let matching_strings: Vec<String> = child.find(&query[str.len()..]);
                        println!("inserting prefix2 {}", str);
                        match &self.value {
                            Some ( prefix ) => {
                                for matching in matching_strings {
                                    let mut new_string = prefix.clone();
                                    (new_string).push_str(&matching);
                                    return_vector.push(new_string);
                                }
                            },
                            None => { return matching_strings; }
                        }
                    }  
                },
                None => {}
            }      
        }

        if self.children.len() == 0{
            match &self.value {
                Some ( str) => {
                    println!("added terminal prefix: {}", str);
                    return_vector.push(str.clone()); 
                },
                None => {}
            }  
        }

        return return_vector;
    }
}

pub fn autocomplete(dictionary: &[String], query: &str) -> Vec<String> {
    let mut tree : RadixNode = RadixNode { children: vec![], value: None};
    for word in dictionary{
        tree.insert(word.clone());
    }
    tree.find(query)
}

fn main() {
    let dictionary = vec![
        "apple".to_string(),
        "banana".to_string(),
        "blueberry".to_string(),
        "cherry".to_string(),
        "date".to_string(),
        "dare".to_string(),
        "bluetail".to_string(),
        "blackmail".to_string(),
        "bluejays".to_string(),
        "arrow".to_string()
    ];
    let query = "";
    let completions = autocomplete(&dictionary, query);
    println!("{:?}", completions);
}
