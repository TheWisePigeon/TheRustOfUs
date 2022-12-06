use std;


fn main() {
    let mut user_input: String = String::new();
    println!("Enter your weight in Kilograms");
    std::io::stdin().read_line(&mut user_input).unwrap();
    let user_weight= user_input.trim().parse::<f32>();
    match user_weight {
        Ok(v)=>{
            println!("Your weight on mars would approximately be {}Kg",calculate_weight_on_mars(v));
            return;
        },
        Err(_)=>{
            println!("You did not enter a valid weight.");
            return;
        }
    }
}
 

fn calculate_weight_on_mars( weight: f32 ) -> f32 {
    ( weight * 9.81 ) / 3.711
}