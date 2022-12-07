use chrono::{DateTime, Utc};
use displaydoc::Display;
use std::net::{AddrParseError, IpAddr};
use thiserror::Error;
use tokio::net::TcpStream;

#[derive(Display, Error, Debug)]
pub enum PeerError {
    /// Can't parse IP Address: {0}
    IpAddressFormat(#[from] AddrParseError),
}

pub struct Peer {
    ip: IpAddr,
    status: PeerStatus,
    pub socket: Option<TcpStream>,
    last_alive: Option<DateTime<Utc>>,
    last_failure: Option<DateTime<Utc>>,
}

impl Peer {
    pub fn new(ip: &String) -> Result<Self, PeerError> {
        Ok(Peer {
            ip: ip.parse::<IpAddr>()?,
            status: PeerStatus::Idle,
            socket: None,
            last_alive: None,
            last_failure: None,
        })
    }

    pub fn ip(&self) -> &IpAddr {
        &self.ip
    }

    pub fn connecting(&mut self) {
        self.status = PeerStatus::OutConnecting;
    }

    pub fn idle(&mut self) {
        self.status = PeerStatus::Idle;
    }

    pub fn handshake(&mut self, is_outgoing: bool) {
        match is_outgoing {
            true => self.status = PeerStatus::OutHandshaking,
            false => self.status = PeerStatus::InHandshaking,
        }
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
