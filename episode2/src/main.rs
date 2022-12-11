
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

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}


fn main() {
    println!("Hello, world!");
    let test = Server::new("127.0.0:5000".to_string());
    test.run();
}
