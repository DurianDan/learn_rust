// https://www.youtube.com/watch?v=ygL_xcavzQ4&t=177s
// from 1:11:27 to 1:19:58

#![allow(unused)]
#![allow(non_snake_case)] // disable warnings for camel cases

fn main(){
    let mut str1: String = String::from("World");
    let str2: String = str1.clone();
    // .clone() can only be used with String, arrays and vector
    // if .clone() isn't used, str1 would be consummed, and be part of the var str2
    // you can not used the var str1 anymore, as it no longer has value in it 
    println!("str2: {}", str2);
    println!("str1: {}", &str1);

    // check changed String
    println!("{}",change_string(&mut str1));
    // has to add "&mut", so the function change_string can modify the parsed string
}

fn change_string(x: &mut String) -> String{
    // &mut has to be called so that the var "x" can be modified
    x.push_str(" is fun");
    return x.to_string();
}