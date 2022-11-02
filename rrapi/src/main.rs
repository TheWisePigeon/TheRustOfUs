use std::io::stdin;
use rand::Rng;


fn main() {
    println!("Welcome to the guessing game");
    let random_number = rand::thread_rng().gen_range(0..5);
    let reader = stdin();
    let mut success = false;
    while !success {
        
        let mut raw_input: String = String::new();
        println!("Enter your guess");
        reader.read_line(&mut raw_input).unwrap();
        let conversion_result = raw_input.trim().parse::<i32>();
        match conversion_result {
            Ok(i)=>{
                if i == random_number {
                    println!("Congratulations!");
                    success = true
                }
                if i > random_number{
                    println!("It's lower")
                }
                if i < random_number{
                    println!("It's higher")
                }
            },
            Err(_)=>{
                println!("You did not enter a valid integer")
            }
        }
    }
}