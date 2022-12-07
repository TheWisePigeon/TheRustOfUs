
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
        println!("I am running baby on {}", self.address);
    }
}

fn main() {
    println!("Hello, world!");
    let test = Server::new(String::from("Yeah"));
    test.run();
}
