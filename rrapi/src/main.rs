use std::io::stdin;
use rand::Rng;


fn main() {
    println!("Welcome to the guessing game");
    let mut user_guess = String::new();
    //Define a variable to generate the random number
    let mut randomNumber = rand::thread_rng().gen_range(0..10);
    println!("Enter your guess");
    stdin().read_line(&mut user_guess);
    println!("{}", user_guess);
}