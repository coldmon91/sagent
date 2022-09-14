use std::time::Duration;
use tokio::sync::mpsc::{self, Sender, Receiver};

use log::{info, warn, debug, error};
use tokio::{
    net::TcpListener, 
    io::{self, AsyncReadExt },
};

mod handlers;
use handlers::*;

pub async fn run(agent_ch_uri : String, viewer_ch_uri : String) -> io::Result<()> {
    println!("agent channel running .. {agent_ch_uri}");
    let ag_listener = TcpListener::bind(agent_ch_uri).await?;

    println!("viewer channel running .. {viewer_ch_uri}");
    let vw_listener = TcpListener::bind(viewer_ch_uri).await?;

    // agent_input_channel, viewer_output_channel
    let (atx, arx) : (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel(256);
    // viewer_input_channel, agent_output_channel
    let (vtx, vrx) : (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel(256);

    let (socket, peer_addr) = ag_listener.accept().await?;
    println!("The agent has entered : {}", &peer_addr.ip().to_string());
    // let ag_handler : AgentHandler = AgentHandler { addr : peer_addr };
    tokio::spawn(async move {
        client_handle(socket, atx, vrx).await;
    });

    let (socket, peer_addr) = vw_listener.accept().await?;
    println!("The viewer has entered : {}", &peer_addr.ip().to_string());
    // let viewer_handler = ViewerHandler { addr : peer_addr };
    tokio::spawn(async move {
        client_handle(socket, vtx, arx).await;
    });

    loop{
        std::thread::sleep(Duration::from_millis(1000));
    }
    return Ok(());
}
