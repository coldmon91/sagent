use std::ops::Add;
use std::str;

use tokio::{self, net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}, sync::mpsc::{Sender, Receiver, self}};

const BUFFER_SIZE: usize = 16384; // 16KB

// for Agent
const command_center_port : usize = 58060;
const rdp_port : usize = 3389;

mod agent;

async fn connect_rdp() -> std::io::Result<TcpStream> {
    let rdp_uri = "127.0.0.1:".to_string().add(&rdp_port.to_string());
    println!("connect to rdp {rdp_uri}");
    let mut rdp_stream = TcpStream::connect(rdp_uri).await.unwrap();
    return Result::Ok((rdp_stream));
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let command_center_uri = "127.0.0.1:".to_string().add(&command_center_port.to_string());
    println!("connect to {command_center_uri}");
    let mut center_stream = TcpStream::connect(command_center_uri).await?;
    let mut rdp_stream = connect_rdp().await.unwrap();

    // input rdp data, out command center 
    let (rtx, rrx) : (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel(256);

    loop {
        let mut centerbuf = vec![0u8; BUFFER_SIZE];
        let mut rdpbuf = vec![0u8; BUFFER_SIZE];
        tokio::select! {
            result = center_stream.read(centerbuf.as_mut_slice()) => { 
                match result {
                    Ok(size) => {
                        if size <= 0 { 
                            println!("socket result size: {}", size);
                            break;
                        } 
                        println!("center stream read : {}", size);
                        rdp_stream.write_all(&centerbuf).await;
                    },
                    Err(e) => {
                        println!("{:?}", e);
                        break;
                    }
                }
            },
            rdp_msg = rdp_stream.read(rdpbuf.as_mut_slice()) => {
                match rdp_msg {
                    Ok(size) => {
                        if size <= 0 { 
                            println!("socket result size: {}", size);
                            break;
                        } 
                        println!("rdp stream read : {}", size);
                        let ret = center_stream.write_all(&rdpbuf).await;
                    },
                    Err(e) => {
                        println!("{:?}", e);
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}