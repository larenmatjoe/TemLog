use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
fn handling(stream :Result<TcpStream, std::io::Error>) {
    match stream {
        Ok(mut stream) => {
//            let ip = stream.peer_addr().unwrap();
//            println!("New Client: {}",ip);
            let _ = stream.write(b"Connected to Server! \n");
            loop {
                let mut buffer = [0;65535];
                let n :usize = match stream.read(&mut buffer) {
                    Ok(n) => n,
                    Err(_) => {
                        break;
                    }
                };
                    
//                let n = n.unwrap();
                if buffer[0] == b'\0' {
//                    println!("{} Disconnected",ip);
                    break;
                }
                if buffer[0] == b'\n' {
                    continue;
                }
                let data = &String::from_utf8_lossy(&buffer)[0..n];
                //println!("Received : {} bytes",data.chars());
                print!("{}",data);
            }
        }
        Err(e) => {
            println!("Error : {}",e);
        }
    }
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    for stream in listener.incoming() {
        let _ = thread::spawn(move || {handling(stream)});
    }
}
