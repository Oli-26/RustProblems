use std::fmt::Display;
use std::fmt;

fn sort_and_print<T: PartialOrd + Copy + Identifiable + Display>(values: Vec<T>) {
    let mut int_vec : Vec<T> = vec![];
    let mut float_vec : Vec<T> = vec![];
    for val in values {
        if val.is_integer() {
            int_vec.push(val);
        } else {
            float_vec.push(val);
        }
    }
    int_vec = merge_sort(int_vec);
    float_vec = merge_sort(float_vec);
    int_vec.append(&mut float_vec);
    for val in int_vec{
        print!("{}, ", val);
    }
}

fn merge_sort<T: PartialOrd + Copy>(values: Vec<T>) -> Vec<T>{
    if(values.len() <= 1){
        return values;
    }
    let midPoint : usize = values.len()/2;
    return combine_sorted_vectors(merge_sort(values[..midPoint].to_vec()), merge_sort(values[midPoint..].to_vec()));
}

fn combine_sorted_vectors<T: PartialOrd + Copy>(vec1 : Vec<T>, vec2 : Vec<T>) -> Vec<T>{
    let mut i = 0;
    let mut j = 0;
    let mut combinedVector : Vec<T> = vec![];
    while(i + j < vec1.len() + vec2.len()){
        if(i >= vec1.len()){
            combinedVector.append(&mut vec2[j..].to_vec());
            return combinedVector;
        }
        if(j >= vec2.len()){
            combinedVector.append(&mut vec1[i..].to_vec());
            return combinedVector;
        }
        if(vec1[i] < vec2[j]){
            combinedVector.push(vec1[i]);
            i += 1;
        }else{
            combinedVector.push(vec2[j]);
            j += 1;
        }
    }
    return combinedVector;
}
fn main() {
    let values: Vec<Number> = vec![
        Number::Integer(1),
        Number::Float(2.5),
        Number::Integer(4),
        Number::Float(3.2),
        Number::Integer(5),
        Number::Float(1.5),
    ];
    sort_and_print(values);
}

#[derive(PartialOrd, PartialEq, Clone, Copy, Debug)]
enum Number {
    Integer(i32),
    Float(f32),
}

impl Identifiable for Number {
    fn is_integer(self) -> bool{
        match self{
            Number::Integer(_) => true,
            Number::Float(_) => false
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Integer(value) => write!(f, "{}", value),
            Number::Float(value) => write!(f, "{:.2}", value),
        }
    }
}
trait Identifiable {
    fn is_integer(self) -> bool;
} 
