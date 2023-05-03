use std::net::{TcpStream, ToSocketAddrs};
use std::env;

fn scan<T: ToSocketAddrs>(addr: T) {
    let stream= TcpStream::connect(addr);
    match stream {
        Ok(_) => {
            println!("Open");
        }
        Err(e) => println!("Closed, {}", e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: <address> <port>");
        std::process::exit(1);
    }
    let address: String = format!("{}:{}", &args[1], &args[2]);
    scan(address);
}
