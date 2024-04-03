use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let number_to_guess = rand::thread_rng().gen_range(0..=100);
    println!("{}", number_to_guess);
    println!("Welcome To Guess The Number");
    println!("Please enter a number between 0 and 100");
    loop {
        let mut your_guess = String::new();
        io::stdin()
            .read_line(&mut your_guess)
            .expect("Please enter a valid number");

        let your_guess: u64 = match your_guess.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        println!("You Guessed: {}", your_guess);
        match your_guess.cmp(&number_to_guess) {
            Ordering::Greater => println!("The number you guessed is greater than expected number"),
            Ordering::Less => println!("The number you guessed is less than expected number"),
            Ordering::Equal => {
                println!("You guessed is equal to the expected number, good job!");
                break;
            }
        };
    }
}
