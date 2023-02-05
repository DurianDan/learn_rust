// https://www.youtube.com/watch?v=ygL_xcavzQ4&t=177s
// from 1:07:55 to 1:11:27

#![allow(unused)]
#![allow(non_snake_case)] // disable warnings for camel cases

use std::ops::Add; 
// this is a trait, an operation/function that can be used for different data types
// best ultitlized for Generics

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T{
    // "T" is a generic type
    // when a function use generics, is must declared <T> after the name of the function
    // this function take 2 generic inputs, and return generic
    
    // generics can only use special functions, called traits
    // has to add "Add<Ouput = T>" to the function name
    // sothat generic x and y can do addition operation (trait) 
    return x+y;
}
fn main(){
    // generics' types can be declared later
    // generics are usefull with functions that interact with multiple data types
    println!("4 +90 = {}", get_sum_gen(5u8,90u8));
}