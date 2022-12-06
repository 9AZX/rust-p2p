use chrono::{DateTime, Utc};
use std::net::IpAddr;
use tokio::net::TcpStream;

pub struct Peer {
    ip: IpAddr,
    status: PeerStatus,
    pub socket: Option<TcpStream>,
    last_alive: Option<DateTime<Utc>>,
    last_failure: Option<DateTime<Utc>>,
}

impl Peer {
    pub fn new(ip: &String) -> Self {
        Peer {
            ip: ip.parse::<IpAddr>().unwrap(),
            status: PeerStatus::Idle,
            socket: None,
            last_alive: None,
            last_failure: None,
        }
    }

    pub fn ip(&self) -> &IpAddr {
        &self.ip
    }
}

pub enum PeerStatus {
    Idle,
    OutConnecting,
    OutHandshaking,
    OutAlive,
    InHandshaking,
    InAlive,
    Banned,
}
