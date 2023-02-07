// https://www.youtube.com/watch?v=ygL_xcavzQ4&t=177s
// from 1:55:45 to 2:05:47

#![allow(dead_code)]

fn main(){
    closures();
}

fn closures(){
    // closures are functions that don't have names
    // like lambda in python ??
    // basic usage
    let can_vote = |age: i32| -> bool{
        age >= 18
    };
    println!("Can vote: {}", can_vote(89));

    // implement "bounds" with "where" keyword
    // use bound "Fn" for generic closures when
    // generic closures can only get reference-type variable
    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where T: Fn(i32,i32) -> i32{
        func(a,b)
    }

    let sum = |a: i32, b: i32| a+b;
    let product = |a: i32,b: i32| a*b;

    println!("4+5 = {}",use_func(4, 5, sum));
    println!("4*5 = {}",use_func(4, 5, product));

}

fn iterators(){
    // iterators
    let mut arr: [i8;4] = [1,2,3,4];
    let mut idx: i32 = 0;
    for val in &mut arr{
        println!("Index {}, value: {}", &idx,val);
        *val *= 2;
        idx += 1;
    }
    println!("{}",arr[1]);
}
