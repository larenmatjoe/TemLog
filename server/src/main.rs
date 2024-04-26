//use std::net::TcpListener, TcpStream};
use std::net::TcpListener;
//use std::thread;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let ip = stream.peer_addr().unwrap();
                println!("New Client: {}",ip);
            }
            Err(e) => {
                println!("Stream Error : {}",e);
            }
        }
    }
}
