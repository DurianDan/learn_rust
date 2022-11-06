// skip warning about unused vaiables
#![allow(unused)]
use std::any::type_name;
use std::io;

fn main() {
    {
        // println!("lesson 1: variables");
        // variables();
        // println!("_____________");
    }
    {
        // println!("lesson 2: data_types");
        // data_types();
        // println!("_____________");
    }
    {
        // println!("lesson3: ");
        // console_input();
        // println!("_____________");
    }
    {
        //print!("lesson 4: Arithmetic and Type Casting: ");
        //arithmetic_type_casting();
        //println!("_____________");
    }
    {
        // println!("lesson 5: conditions and control_flow: ");
        // conditions_control_flow();
        // println!("_________________________")
    }
    {
        // println!("Lesson 6: Functions, Expressions, Statements");
        // println!("____________________");
        // functions();
    }
    my_own();
}

fn print_type_of<T>(_: &T) {
    // A function to get the type of the variable
    // the type of a var is not preserved after compiling
    // so, when creating the ouput, this function returns the best guess,
    // not 100% correct, and meant for debugging
    println!("{}", std::any::type_name::<T>())
}

fn variables() {
    // let -> a variable that cannot change type
    // x can only be string in the future
    // mut -> Allow x to be changed
    // all var in rust are immutable by default
    let mut x = "ok_variables";
    println!("this is the first {}", x);
    x = "var changed";
    println!("{}", x);

    {
        // this is a scope !!!
        // every variable in this scope can only exist in it
        let y = "this is a variable inside a scope";
        // declare a string var with a type indicator &str
        println!("{}", y);
    }

    // or, you can redeclare the variable...
    // ...when dont want the variable to be mutable
    let y = 10;
    println!("number variable is {}", y);
    let y = "pl";
    println!("number variable has been changed to {}", y);
}
fn data_types() {
    //primitive data types
    // I. scalar type: single var like an int, float, string
    {
        // I.1: Integer:
        let x: i32 = -2;
        // integer is i32 by default, storing this variable in 32 bits
        // other int var: i8,i16,i32,i64,128 (signed, or can be negative)
        let y: u8 = 4;
        // u8,u16,u32,u64,u128 -> unsigned, or can not be negative (cannot have sign)

        // I.2: others
        let floating_point: f32 = 19.9; // Floating point
        let boolean: bool = true; //boolean type
        let letter: char = 'a';
        let sentence: &str = "oko";
    }
    // II. compound type: tuple and array
    {
        // II.1. Tuple
        {
            // Allow this tuple to be mutale
            let mut temp_tup: (i8, bool, char, &str) = (1, true, 's', "last_var");
            // update the tuple
            temp_tup.3 = "last_var changed, but can only be the same type: &str";
            // cannot add more elements
            // cannot update element with different type
            temp_tup = (2, false, 'o', temp_tup.3);
            // print the element with the index 1 in the temp_tup
            println!("{}", temp_tup.3);
        }
        {
            // II.2. Array
            // Elements Can only have the same types
            let mut arr: [i8; 5] = [1, 2, 3, 4, 5];
            arr[1] = arr[0];
            println!("this is a sum from an array: {}", arr[1] + arr[4]);
            // initial an empty array
            let arr2: [i64; 8];
        }
    }
}
fn console_input() {
    println!("Hello fren !!");
    // Initialize an emty string with the function new()
    // of the "String" crate
    let mut input: String = String::new();
    // If put only the var name into the function...
    // function with use a copy of the value asigned to the var
    // has to add "&mut" to ensure that we are reference to the var itself
    // this method can only takin string input
    // the "input" var has to be of "String" type
    io::stdin()
        .read_line(&mut input)
        .expect("this is wrong type of input");
    println!("{}", input);
}
fn arithmetic_type_casting() {
    // I. basic Arithmetic
    {
        let x: i8 = 9;
        let y: i8 = 5;
        let o: f32 = 9.3;
        // can only perform operation on the same integer type,
        // the result can only be of the same type
        // when devide an integer, like i8, the result can only a integer
        // if it's a float, the result is the part before the dot.
        println!("\n{} {}", x % y, x / 2);

        // the code below, result in an Overflow,
        // meaning that the ouput is to big to be stored as an i8 integer
        println!("{}", x * y); //*y);
    }

    // II. Type Casting, Operation on different type
    {
        let x: i32 = 89;
        let y: f64 = 90.3;
        // change x into f64 (same as y)
        // to perfrom arithmetic
        println!("{}", (x as f64) / y);
        // always change to an int type as big or
        // bigger than current var type
    }

    // III. Get integet from user input
    {
        //let mut temp_input: String = String::new();
        // io::stdin().read_line(&mut temp_input).expect("This is a wrong format for input");
        let mut temp_input: String = String::new();
        io::stdin().read_line(&mut temp_input);
        // change string into int types
        // trim() remove the excess spaces or escapes at the end and start of a string
        // parse() get the data out of a type (my understanding)
        // unwrap() return the data in the prefered type (my understanding)
        let mut int_input: i32 = temp_input.trim().parse().unwrap();
        println!("{}", int_input + 1);
    }
}
fn conditions_control_flow() {
    // I. Conditions
    {
        let x: i32 = 3;
        let y: i8 = 8;
        let temp_compare: bool = x > y as i32;
        println!(
            "{} {} {} {}",
            (x > y as i32 || 0 == 0),
            !temp_compare,
            (x >= 3 && x == 3),
            y as f32 / 4.0 == 2.0
        );
    }

    // II. Control Flow
    {
        let mut food: String = String::new();
        io::stdin()
            .read_line(&mut food)
            .expect("this is not how you input");

        let food: &str = food.trim();
        print_type_of(&food);
        if food == "durian" {
            println!("ok! i like durian");
        } else if food == "vegetables" {
            println!("That's ok too");
        } else {
            println!("i dont know what you are talking about");
        }
    }
}

// functions() is a statement, it do something, but not returning anything
// expression will return something, and it may or may not effect the var parsed
fn functions() {
    // I. Functions
    {
        fn inside_function(x: i32,y: i8) -> i32{
            if 8 == 8 {
                return x + y as i32;
            }
            else{
                return x - y as i32;
            }
        }
        println!("{}",inside_function(2,8));
    }

    // II. Statement vs Expression
    {
        // This is a expression, as it return x : i32
        let x: i32 = {
            let mut x: String = String::new();

            io::stdin()
                .read_line(&mut x)
                .expect("this is not dae way");

            x.trim().parse().unwrap()
        };

    }
}
fn my_own(){
    let mut x: i32 = {
        let mut x: String = String::new();
        io::stdin()
            .read_line(&mut x)
            .expect("This is not de wae");

        x.trim().parse().unwrap()
    };

    println!("this is an i32 variable: {}",x);
}