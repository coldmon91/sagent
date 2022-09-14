use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::{self, Sender, Receiver};

const BUFFER_SIZE: usize = 16384; // 16KB

pub trait Handler {
    
}

pub async fn client_handle(mut socket : tokio::net::TcpStream, tx : Sender<Vec<u8>>, mut rx : Receiver<Vec<u8>>) -> tokio::io::Result<()> {
    loop {
        let mut frame_buf = vec![0u8; BUFFER_SIZE];
        tokio::select! {
            result = socket.read(frame_buf.as_mut_slice()) => { 
                let size = result.unwrap();
                if size <= 0 { 
                    println!("socket result size: {}", size);
                    break;
                } 
                println!("socket read {size}");
                tx.send(frame_buf[0..size].to_vec()).await; 
            },
            result = rx.recv() => {
                match result {
                    Some(frame) => {
                        println!("channel read {}", &frame.len());
                        socket.write_all(&frame).await;
                    },
                    None => {
                        println!("disconnected");
                        break;
                    }
                }
            }
        }
    }
    return Ok(());
}

pub struct AgentHandler {
    addr : std::net::SocketAddr,
}
impl Handler for AgentHandler{

}

pub struct ViewerHandler {
    addr : std::net::SocketAddr,
}
impl Handler for ViewerHandler{
    
}