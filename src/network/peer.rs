use std::net::IpAddr;
use chrono::{DateTime, Utc};
use tokio::net::TcpStream;

pub struct Peer {
    ip: IpAddr,
    socket: TcpStream,
    status: PeerStatus,
    last_alive: Option<DateTime<Utc>>,
    last_failure: Option<DateTime<Utc>>
}

enum PeerStatus {
    IDLE,
    OUTCONNECTING,
    OUTHANDSHAKING,
    OUTALIVE,
    INHANDSHAKING,
    INALIVE,
    BANNED,
}