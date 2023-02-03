use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    let guess: i32 = {
        let mut x: String = String::new();
        println!("Please enter your guess number: \n");
        io::stdin()
            .read_line(&mut x)
            .expect("this is not dae wae");

        x.trim().parse().unwrap()
    };

    let secret_number: i32 = rand::thread_rng()
                            .gen_range(1..=10);
    
    match guess.cmp(&secret_number){
        Ordering::Less => println!(
            "Your guess {} is fewer than the result {}",
            guess,
            secret_number),
        Ordering::Equal => println!(
            "Your guess is equal to the result {}",
            secret_number),
        Ordering::Greater => println!(
            "Your guess {} is greater than the secret_number {}",
            guess,
            secret_number
        )
    };
}