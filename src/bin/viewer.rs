use std::{ops::Add, io::stdin};
use std::str;

use tokio::{self, net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};

const BUFFER_SIZE: usize = 16384; // 16KB

// for Viewer
const command_center_port : usize = 58070;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let command_center_uri = "127.0.0.1:".to_string().add(&command_center_port.to_string());
    println!("connect to {command_center_uri}");
    let mut stream = TcpStream::connect(command_center_uri).await?;

    let mut buf = vec![0u8; BUFFER_SIZE];
    loop {
        let mut input_string = String::new();
        stdin().read_line(&mut input_string)
            .ok()
            .expect("Failed to read line");
        let len = input_string.len()-1;
        stream.write_all(input_string[0..len].as_ref()).await?;

        match stream.read(&mut buf).await {
            Ok(size) => {
                if size <= 0 { 
                    println!("socket result size: {}", size);
                    break;
                } 
                println!("viewer read : {}", size);
                let s = match str::from_utf8(&buf) {
                    Ok(v) => {
                        println!("reply {}", v);
                    },
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
            },
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }
    Ok(())
}