fn get_arguments() -> Vec<String> {
    std::env::args().collect()
}

fn main() {
    println!("Hello, world!");
    let args = get_arguments();
    println!("{:?}", args);
}
