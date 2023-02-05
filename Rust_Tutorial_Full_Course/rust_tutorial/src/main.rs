// https://www.youtube.com/watch?v=ygL_xcavzQ4&t=177s
// from 1:19:58

#![allow(unused)]
use std::collections::HashMap;

fn main(){
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
    for (hero, name) in heroes_name.iter(){
        println!("Real name of {} is {}", hero, name);
    }
}
