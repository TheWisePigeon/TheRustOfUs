
struct Server{
    address: String
}

impl Server {
    fn new(address: String) -> Self {
        Self { 
            address 
        }
    }

    fn run(self) {
        println!("I am running baby");
    }
}

fn main() {
    println!("Hello, world!");
}
