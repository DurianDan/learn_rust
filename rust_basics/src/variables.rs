fn main() {
    // let -> a variable that cannot change type
    // x can only be string in the future
    // mut -> Allow x to be changed 
    // all var in rust are immutable by default
    let mut x = "ok_variables"; 
    println!("this is the first {}",x);
    x = "var changed";
    println!("{}",x);

    // or, you can redeclare the variable...
    // ...when dont want the variable to be mutable
    let y = 10;
    println!("number variable is {}",y);
    let y = "pl";
    println!("number variable has been changed to {}",y);
}
