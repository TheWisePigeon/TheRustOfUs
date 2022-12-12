use std::net::{TcpListener, TcpStream};
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self) {
        println!("I am running baby! On {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((Stream, SocketAddress)) => {

                },
                Err(e) =>  println!("Failed to accepts incoming connection {}", e),
            }
        }
    }
}
