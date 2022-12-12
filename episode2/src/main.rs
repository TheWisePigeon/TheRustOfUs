use server::Server;
// use http::{Method, Request };

mod server;
mod http;


fn main() {
    let test = Server::new("127.0.0.1:5000".to_string());
    test.run();
}
