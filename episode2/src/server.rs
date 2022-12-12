use std::{net::{TcpListener}, io::Read};
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self) {
        println!("I am running on {} baby", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read( &mut buf) {
                        Ok(v) =>{
                            println!("{}", v);
                            println!("{}", String::from_utf8_lossy(&buf));
                        },
                        Err(e)=>{
                            println!("Failed to read from socket {}", e);
                        }
                    } ;
                },
                Err(e) =>  println!("Failed to accepts incoming connection {}", e),
            };
        }
    }
}
