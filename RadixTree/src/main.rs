use std::any;

#[derive(Debug, Default, Clone)]
struct RadixNode {
    children: Vec<RadixNode>,
    value: Option<String>,
}

fn find_longest_matching_prefix(str1 : String, str2 : String) -> String{
    let mut longest_matching_prefix: String = String::new(); 
    let mut chars2 = str2.chars();
    for c in str1.chars() {
        match chars2.next() {
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
    longest_matching_prefix
}

fn print_radix_node(node: &RadixNode, prefix: &str) {
    let value = match &node.value {
        Some(s) => format!(" - {}", s),
        None => "".to_string(),
    };
    println!("{}{}{}", prefix, node.children.len(), value);
    for child in &node.children {
        print_radix_node(child, &(prefix.to_owned() + "  "));
    }
}

impl RadixNode {
    fn have_children(&self) -> bool {
        self.children.len() != 0
    }

    fn insert_to_child_if_char_matches(&mut self, to_match : String){
        let mut any_matches = false;
        for child in self.children.iter_mut(){
            match child.value.as_ref(){
                Some( child_prefix ) => {
                   if child_prefix.chars().next() == to_match.chars().next(){
                        any_matches = true;
                        child.insert(to_match.to_string());
                        break;
                   } 
                },
                _ => {}
            }
        }
        if !any_matches{
            self.add_child(vec![], Some(to_match.to_string()));
        }
    }

    fn add_child(&mut self, children : Vec<RadixNode>, value: Option<String>){
        self.children.push(RadixNode { children: children, value: value });
    }

    fn insert(&mut self, value_to_add: String) {
        let val = &self.value.clone();
        match val {
            Some (prefix) => {
                let longest_matching_prefix: String = find_longest_matching_prefix(prefix.clone(), value_to_add.clone());
                let left_over_for_insert_value = value_to_add[longest_matching_prefix.len()..].to_string();
                if longest_matching_prefix == *prefix {
                    self.insert_to_child_if_char_matches(left_over_for_insert_value);
                }else if longest_matching_prefix.len() > 0{                    
                    self.value = Some(longest_matching_prefix.to_string());                    
                    let child_copy = self.children.clone();
                    self.children = vec![];
                    self.add_child(child_copy, Some(prefix[longest_matching_prefix.len()..].to_string()));
                    self.add_child(vec![], Some(value_to_add[longest_matching_prefix.len()..].to_string()));
                }else{
                    self.value = None;
                    let child_copy = self.children.clone();
                    self.children = vec![];
                    self.add_child(child_copy, Some(prefix.to_string()));
                    self.add_child(vec![], Some(value_to_add.to_string()));
                }
            },
            None => {
                self.insert_to_child_if_char_matches(value_to_add);   
            }
        }
    }

    fn find_all_and_collect(&self, query: &str, prefix: String) -> Vec<String>{
        let mut collected_strings: Vec<String> = vec![];
        let child_collection = self.find(&query);
        for child_string in child_collection {
            let mut new_str = prefix.clone();
            new_str.push_str(&child_string);
            collected_strings.push(new_str);
        }
        return collected_strings;
    }
    fn find(&self, query: &str) -> Vec<String>{
        let mut collected_strings: Vec<String> = vec![];

        if !self.have_children(){
            if(query == "" || query == self.value.clone().unwrap()){
                return vec![self.value.clone().unwrap()];
            }
        }

        match self.value.as_ref() {
            Some ( inner_value ) => {
                let mut prefix = inner_value.chars();
                let mut all_match = true;
                for query_char in query.chars() {
                    match prefix.next() {
                        Some ( prefix_char ) => {
                            if(query_char != prefix_char){
                                all_match = false;
                            }else{
                            }
                        },
                        None => { break; }
                    }
                }
                if all_match {
                    if(query.len() > inner_value.len()){
                        for child in self.children.iter(){
                            collected_strings.append(&mut child.find_all_and_collect(&query[inner_value.len()..], inner_value.clone()));
                        };
                    }else{
                        for child in self.children.iter(){
                            collected_strings.append(&mut child.find_all_and_collect("", inner_value.clone()));
                        };
                    }
                }
            },
            None => {
                for child in self.children.iter(){
                    collected_strings.append(&mut child.find_all_and_collect(query, String::new()));
                };
            }
        }
        return collected_strings;
    }
}

pub fn autocomplete(dictionary: &[String], query: &str) -> Vec<String> {
    let mut tree : RadixNode = RadixNode { children: vec![], value: None};
    for word in dictionary{
        tree.insert(word.clone());
    }
    
    print_radix_node(&tree, "");
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
        "bluil".to_string(),
        "blueferfw".to_string(),
        "blackmail".to_string(),
        "bluejays".to_string(),
        "arrow".to_string()
    ];
    let query = "";
    let completions = autocomplete(&dictionary, query);
    println!("{:?}", completions);
}
