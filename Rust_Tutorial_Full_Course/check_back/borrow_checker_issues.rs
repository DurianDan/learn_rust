fn main(){
    // .split_whitespace()
    let st1: String = String::from("F second_word");
    let st2 = st1.replace("F", "first_word");
    // error: let st2 = st1.replace("F", "first_word").split_whitespace();

    for w in st2.split_whitespace(){
        println!("{}",w);
    }
    // slice the String
    // error: println!("{}", st3[0..3]);
    println!("{}", &st1[0..3]);
    
}
