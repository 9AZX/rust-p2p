use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ChannelMessage {
    Handshake,
    Alive,
    AskPeersList,
    PeersList(String),
    Close
}