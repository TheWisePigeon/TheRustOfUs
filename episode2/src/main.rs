use server::Server;
use http::{Method, Request };

mod server;
mod http;


fn main() {
    println!("Hello, world!");
    let test = Server::new("127.0.0:5000".to_string());
    test.run();
}
