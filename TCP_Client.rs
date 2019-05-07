use std::net::TcpStream;
use std::io::{Read,Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("google.com:80") {
        Ok(mut stream) => {
            println!("Connected To Server.");
            let input = b"TEST";
            stream.write(input).unwrap();
            println!("Message Sent");
            let mut recv = [0 as u8; 6]; //6bytes input
            println!("Waiting for recv");
            stream.read_exact(&mut recv);
            let text = from_utf8(&recv).unwrap();
            println!("recv is {}",text);
        },
        Err(e) => {
            println!("failed to  {}",e);
        }
    }
}
