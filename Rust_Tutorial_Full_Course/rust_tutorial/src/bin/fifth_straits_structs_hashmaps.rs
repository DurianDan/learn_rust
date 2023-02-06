// https://www.youtube.com/watch?v=ygL_xcavzQ4&t=177s
// from 1:19:58 to 1:34:20

#![allow(unused)]
use std::collections::HashMap;

fn main(){
    traits();
}

fn traits(){
    // using/creating the same trait for different structs
    // trait taking care parsing same data types to different structs with different data types requirements
    // trait also taking care of the ouput, so that it has the intuatively same result  
    trait Greeting{
        fn new(name: String, position: String, age: i16) -> Self;
        // because trait can work with multiple structs,
        // has to have a function (new() in this example) that take in variables and return
        // the struct with suitable datatypes(generics)
        fn message(&self) -> String;
    }
    // create 2 different structs, with different data types
    struct Wakandata{
        name: String,
        position: String,
        team: String,
        age: i16
    }
    struct SBC{
        name: String,
        position: String,
        division: String,
        age: i32
    }
    // create frunctions from trait for different structs
    impl Greeting for SBC {
        fn new(name: String, position: String , age: i16) -> SBC{
            return SBC{
                name:name, position:position, age: age as i32,
                division: String::from("SBC")}; 
                // add custom input "SBC" that the new() function doesn't have
                // change datatypes i16 to i32 of var "age"
        }
        fn message(&self) -> String{
            return format!("{} is {} years old, working for {}, at position {}", 
                            self.name, self.age, self.division, self.position);
        }
    }
    impl Greeting for Wakandata {
        fn new(name: String, position: String , age: i16) -> Wakandata{
            return Wakandata{
                name:name, position: position, age: age, 
                team: String::from("Wakandata") ,
                };
                // add custom input "Wakandata" that the new() function doesn't have
        }
        fn message(&self) -> String{
            return format!("{} is {} years old, working for {}, at position {}", 
                            self.name, self.age, self.team, self.position);
        }
    }

    // use created traits of 2 functions
    let huynv: Wakandata = Greeting::new("Huy".to_string(), "Data Assistant".to_string(), 23);
    let linhnt: SBC = Greeting::new("Linh".to_string(), "Data Leader".to_string(), 24);

    println!("{}", huynv.message());
    println!("{}", linhnt.message())
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
