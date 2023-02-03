#![allow(unused)]
#![allow(non_snake_case)] // disable warnings for camel cases

use std::io;
use std::io::{Write,BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

use rand::Rng;

fn main(){
    let input_age: i32 = {
        let mut temp_x: String = String::new();
        io::stdin().read_line(&mut temp_x) 
                    .expect("This is not a valid input");
        temp_x.trim_end().parse().unwrap()
    };

    if (input_age < 18) || (input_age > 80) {
        println!("You can't vote");
    } else {
        println!("You can vote");
    };

}


fn TWO_math_and_constants(){
    // constants are embeded in the binary file (compile-time evaluation)
    // immutables are generated when running the compiled binary file (run-time computed)
    
    // constants types must be declared
    
    // constants can be declared at any scope, even the global scope

    const SALARY: i32 = 1500; // all chars capitalized are naming convention for const 
    const PI: f32 = 3.141528;
    const MONTH: i8 = 6; 
    
    let age: &str = "23"; // double quotes for strings, single quote for char
    let mut age: u32 = age.trim().parse() // trim remove all spaces
        .expect("Age wasn't assigned a number"); 
        // expect is for error handling, when function return "Ok" and "Err"
    age = age + 1;
    println!("I'm {} years old, and I want {} in month {}", &age, &SALARY, &MONTH + (3/4));
    // "5/4" returns 1 because this is the division of 2 i32 and will only return an i32
    // which is the integer part of the result
}

fn ONE_input_and_expect() {
    let mut greeting: &str = "Nice to meet you";

    let input_name: String = {
        let mut temp_x: String = String::new();
        io::stdin().read_line(&mut temp_x)
            .expect("Didn't Receive Input");
        
        temp_x.trim_end().to_string()
    };

    println!("Hello, {}! {}", &input_name, greeting);
}