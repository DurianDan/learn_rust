// https://www.youtube.com/watch?v=ygL_xcavzQ4&t=177s

#![allow(unused)]
#![allow(non_snake_case)] // disable warnings for camel cases

use std::io;
use std::io::{Write,BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

use rand::Rng;

fn main(){ 
    // Tuple can't be mutable
    // every element's type needs to declared before hand 
    let my_tuple: (u8,String, f32) = (2,"Huy NV".to_string(), -90.8);

    // get element with index is different from that of arrays
    // the index must appear after a dot
    println!("My employee code is {}", my_tuple.1);

    let (v1,v2,v3) = my_tuple;
    println!("This is the third element: {}", v3);

}

fn FIVE_arrays(){ // arrays can be mutable
    let mut arr: [i8;5] = [3,4,5,80,7];
    let mut loop_idx: usize = 0;

    let [e1,e2,e3,e4,e5] = arr;
    println!("This is the second element: {}", e2);

    // "while" loop, can be use like "loop" loop
    while (loop_idx < arr.len()) {
        if (arr[loop_idx] % 2 == 0){
            println!("Even number: {} at index {}", &arr[loop_idx],&loop_idx);
        };
        loop_idx += 1;
    };

    // for loop
    for i in arr.iter(){ 
        // .iter() creates an iterator out of an array
        println!("Val :{}", &i);
    }
}

fn FOUR_match_statement(){
    // match number to range
    let my_age: i32 = 23;
    match my_age {
        1..=4 => println!("You are a Toddler ar best"),
        // 1..4 = [1,2,3]
        // 1..=4 = [1,2,3,4]
        15 | 23 => println!("You are going through crysis"),
        18 => println!("You can vote"),
        65..=i32::MAX => println!("You are freaking old"),
        _ => println!("You are a human")
        // the underscore _ is for any other result of the match statement
    };

    // comparing match
    let input_salary: i32 = {
        let mut temp_x: String = String::new();
        io::stdin().read_line(&mut temp_x)
                    .expect("This is not a valid number");
        temp_x.trim_end().parse().unwrap()
    };

    match input_salary.cmp(&1000){ //compare
        Ordering::Equal => println!("This is decent"),
        Ordering::Less => println!("This is too low"),
        Ordering::Greater => println!("This is desireable")
    }

}

fn THREE_random_ifstatement(){
    // random number
    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    println!("This is the random number: {}", &random_num);

    // If statement
    let input_age: i32 = {
        let mut temp_x: String = String::new();
        io::stdin()
            .read_line(&mut temp_x) 
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