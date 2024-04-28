use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use sqlite;
fn database(data :String) {
    let _connection = sqlite::open("storage.db").unwrap();
    let _query = "insert into";
}
fn handling(stream :Result<TcpStream, std::io::Error>) {
    match stream {
        Ok(mut stream) => {
            let ip = stream.peer_addr().unwrap();
            println!("New Client: {}",ip);
            let _ = stream.write(b"Connected to Server!");
            loop {
                let mut buffer = [0;65535];
                let _ = stream.read(&mut buffer);
                if buffer[0] == b'\0' {
                    println!("{} Disconnected",ip);
                    break;
                }
                if buffer[0] == b'\n' {
                    continue;
                }
                let data = &String::from_utf8_lossy(&buffer)[0..4];
                //println!("Received : {} bytes",data.chars());
                println!("Payload : {}",data);
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
