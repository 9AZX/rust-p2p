use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use tokio::net::TcpStream;

#[derive(Debug)]
pub enum ChannelMessage {
    Connection {
        ip: IpAddr,
        socket: TcpStream,
        is_outgoing: bool,
    },
    Handshake,
    Alive,
    AskPeersList,
    PeersList(String),
    Close,
}
