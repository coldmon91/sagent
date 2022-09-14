use std::ops::Add;

mod command_center;


/* Agent channel port */
const agent_ch_port : usize = 58060;
/* Viewer channel port */
const viewer_ch_port : usize = 58070;

#[tokio::main]
async fn main() {

    let agent_ch_uri = "0.0.0.0:".to_string().add(&agent_ch_port.to_string());
    let viewer_ch_uri = "0.0.0.0:".to_string().add(&viewer_ch_port.to_string());
    command_center::run(agent_ch_uri, viewer_ch_uri).await.unwrap();

    // let uri = "127.0.0.1:4486".to_string();
}
