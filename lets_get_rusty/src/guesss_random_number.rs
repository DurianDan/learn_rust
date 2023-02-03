use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // taking in user input
    println!("Lets play a guessing game");
    println!("Please ENTER your guess number: ");
    let guess: i32 = {
        let mut x: String = String::new();

        io::stdin()
            .read_line(&mut x)
            .expect("thats not dae way");
        // turn string input into an integer
        x.trim().parse().unwrap()
    };

    // Get the random number
    let secret_number: i32 = rand::thread_rng()
                                    .gen_range(1..=100);

    // compare values using .cpm() -> return 3 enum 
    match guess.cmp(&secret_number) {
        Ordering::Less => println!(
            "Your guesss is {} fewer than the number {}",
            secret_number - guess,
            secret_number),
        Ordering::Equal => {
            println!(
            "Your guesss is equal to the number {}",
            secret_number)
            },
        Ordering::Greater => println!(
            "Your guess is {} greater than the number {}",
            guess - secret_number,
            secret_number
        )
    }
}
