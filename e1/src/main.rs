fn main() {
    println!("Hello, world!");
    let japan = "Li hao combi";
    let espagnol = "Hola amigo";
    let countries = [japan, espagnol];
    for country in countries.iter() {
        println!("{}", &country);
    }
}
