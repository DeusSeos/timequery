use std::io::{Read};
use std::net::TcpStream;
use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} host:port", args[0]);
        return;
    }
    let addr = &args[1];
    
    match TcpStream::connect(addr) {
        Ok(stream)=> {
        // println!("Connected to server!");
        let mut stream = stream;
        let mut buffer = [0; 512];
        let mut result = String::new();
        while stream.read(&mut buffer).unwrap() > 0 {
            match std::str::from_utf8(&buffer) {
                Ok(v) => result.push_str(v),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
        }
        
        println!("{}", result);
    },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }

    }
    
}

