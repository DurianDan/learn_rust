// https://www.youtube.com/watch?v=ygL_xcavzQ4&t=177s
// from 1:19:58

#![allow(unused)]
use std::collections::HashMap;

fn main(){
    simple_structs();
}

fn traits(){
    struct Employee<S,U>{
        name: S,
        division_team: S,
        position: S,
        age: U
    }
    trait Shape{
        fn new(name: String, division_team: String, position: String) -> Self;
        // because trait can work with multiple data types
        // has to have a function (new()) that takin varables and return that struct
        // the 
        fn message(&self) -> String;
    }
}

fn simple_structs(){
    // structs are custom data types
    // a struct has multiple field, an enum has multiple variants
        // while concrete value of a struct type has multiple fields
        // a concrete value of an enum type is exactly one variant
    struct Employee<S,U>{
        // initialize struct with 2 generics
        name: S,
        division_team: S,
        position: S,
        age: U
    }
    // basic usage
    let mut huynv: Employee<String, i8> = Employee {
         name: String::from("Huy NV"),
         division_team: String::from("Wakandata"),
         position: String::from("Data Assistant"),
         age: 23i8
        };
    // change the field name, can only be done 
    // when the struct "huynv" is mutable
    huynv.name = String::from("Nguyễn Vũ Huy");
    println!("{} is {} years old, working at position {}, from team {}.",
            huynv.name, huynv.age, huynv.position, huynv.division_team); 
}

fn hashmaps(){
    // hashmaps
    let mut heroes_name: HashMap<&str, &str> = HashMap::new();
    heroes_name.insert("Endeavor", "Enji Todoroki");
    heroes_name.insert("Batman", "Bruce Wayne");
    heroes_name.insert("Mugiwara", "Monkey D. Luffy");

    println!("number of heroes is {}", heroes_name.len());
    println!("Spiderman is in the list: {}",
            heroes_name.contains_key(&"Spiderman"));
    println!("The realname of Endeavor is {:?}", 
            heroes_name.get(&"Endeavor"));
            // has to format with {:?}, because the .get functions return Some() or None
            // which don't have Display traits.
            // {:?} helps pretty-print
    for (hero, name) in heroes_name.iter(){
        println!("Real name of {} is {}", hero, name);
    }
}
