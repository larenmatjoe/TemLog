use std::net::{TcpListener, TcpStream};
//use std::thread;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
            
            }
            Err(e) => {
                println!("Stream Error : {}",e);
            }
        }
    }
}
