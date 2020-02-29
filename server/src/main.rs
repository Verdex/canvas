
use std::net::{TcpListener, TcpStream};

fn main() {

    let maybe_listener = TcpListener::bind("127.0.0.1:3000");
    match maybe_listener {
        Err(e) => println!("failed to bind {}", e),
        Ok(listener) => println!("Bound successfuly"),
    }
}
