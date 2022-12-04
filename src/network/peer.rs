use std::net::IpAddr;
use chrono::{DateTime, Utc};
use tokio::net::TcpStream;
use serde::{Deserialize, Serialize};

pub struct Peer {
    ip: IpAddr,
    status: PeerStatus,
    socket: Option<TcpStream>,
    last_alive: Option<DateTime<Utc>>,
    last_failure: Option<DateTime<Utc>>
}

impl Peer {
    pub fn new(ip: &String) -> Self {
        Peer {
            ip: ip.parse::<IpAddr>().unwrap(),
            status: PeerStatus::IDLE,
            socket: None,
            last_alive: None,
            last_failure: None
        }
    }
}

pub enum PeerStatus {
    IDLE,
    OUTCONNECTING,
    OUTHANDSHAKING,
    OUTALIVE,
    INHANDSHAKING,
    INALIVE,
    BANNED,
}