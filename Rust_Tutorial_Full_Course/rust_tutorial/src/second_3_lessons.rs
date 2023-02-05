// https://www.youtube.com/watch?v=ygL_xcavzQ4&t=177s
// from 51:50 to 1:07:55

#![allow(unused)]
#![allow(non_snake_case)] // disable warnings for camel cases

use std::io;
use std::io::{Write,BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

fn ELEVEN_vector_to_string(vector: &[i8]) -> String{
    // &Vec<i8> can only be referenced to a vector
    //  &[i8] ca be referenced to a vector or an array 
    let mut ouput_string: String = String::from("[");
    for val in vector{
        ouput_string.push_str(
            format!(" {},",val) // create a String
                    .as_str() // turn into a str
        )
    };
    ouput_string.push(']');
    return ouput_string
}
fn main(){
    let test_vec = vec![1,2,3];
    println!("{}",vector_to_string(&test_vec))
}



fn TEN_vector(){
    // Vector is like array, but can only store variables of the same type 
    /// initialize a vector
     let vec1: Vec<u32> = Vec::new();
     let mut vect2: Vec<u32> = vec![1,2,3,4];
     vect2.push(5); // add an element to the end of the vector
    
    /// check if an element exist in a vector
    let check_idx: usize = {
        let mut temp_x: String = String::new();
        io::stdin().read_line(&mut temp_x)
                .expect("This is not a valid input number");
        temp_x.trim_end().parse().unwrap()
    };
    match vect2.get(check_idx) {
        Some(_) => println!("value is: {}", &vect2[check_idx]),
        None => println!("There isn't a value at index {}", check_idx)
    };
    /// loop through the vector
    for val in &mut vect2{ // add &mut so that the function can modify the vector
        *val = *val* 2;
        println!("{}",val) 
        // val is actually referencing a variable in the vector
        // to dereference, add an asterisk before "val"
        // so now you can use val to modify content of the vector
    }

    /// pop and remove
    println!("{} is the removed element at the index 2",vect2.remove(2));
    println!("{:?} is the removed element at the index -1",vect2.pop())

}

fn NINE_enums(){
    // enums are custom datatypes 
    // that can only have a limited set of potential values
    enum Family { // create an enum
        Huy, HaiAnh, Me, Bo
    };
    impl Family { // create functions for the enum
        fn employment(&self) -> &str{ // taking in the enum as input
            match self {
                Family::HaiAnh => "Hai Anh is not employed right now",
                Family::Bo => "He is selling rice",
                Family::Me => "She is an electricity worker",
                Family::Huy => "He is an Data Analyst"
            }
        }
        fn age(&self) -> i8{
            match self {
                Family::HaiAnh => 20,
                Family::Bo => 49,
                Family::Me => 49,
                Family::Huy => 23
            }
        }
    }
    // implement the enum
    let family_member: Family = Family::Bo;
    println!("Bo is {} years old", family_member.age()); 
    println!("{}", family_member.employment()); 
}