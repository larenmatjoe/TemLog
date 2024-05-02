use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use sysinfo::Networks;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut connection = TcpStream::connect("127.0.0.1:9000").await?;
    let networks = Networks::new_with_refreshed_list();
    let mut string = String::new();
    for (interface_name, data) in &networks {
       // println!("{interface_name}: {} B (down) / {} B (up)",data.total_received(),data.total_transmitted());
        let temp = String::from(format!("{interface_name} : {} B (down) / {} B (up) \n",data.total_received(),data.total_transmitted()));
        string.push_str(&temp);
    }
    let _ = connection.write_all(string.as_bytes()).await?;
    println!("{}",string);
   // let _ = connection.write_all(b"\0").await?;
    Ok(())
}
