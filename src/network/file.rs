use crate::error_logger::InspectErr;
use crate::network::peer::{Peer, PeerError};
use displaydoc::Display;
use log::{info, warn};
use std::collections::HashMap;
use std::net::{AddrParseError, IpAddr};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{fs, io};
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Display, Error, Debug)]
pub enum PeersFileControllerError {
    /// Unable to read peers json file
    Io(#[from] io::Error),
    /// JSON does not have correct format.
    Serialization(#[from] serde_json::Error),
    /// Can't parse IP Address: {0}
    IpAddressFormat(#[from] AddrParseError),
    /// Impossible to acquire lock over a RwLock
    RwLockPoisoned,
    /// Error with a peer: {0}
    Peer(#[from] PeerError),
}

#[derive(Default)]
pub struct PeersFileController {
    file_path: String,
    is_changed: AtomicBool,
}

impl PeersFileController {
    pub fn new(file: &str) -> PeersFileController {
        PeersFileController {
            file_path: file.to_string(),
            is_changed: AtomicBool::new(false),
        }
    }

    pub fn changed(&self) {
        self.is_changed.store(true, Ordering::SeqCst);
    }

    fn parse_peer(data: String) -> Result<HashMap<IpAddr, Peer>, PeersFileControllerError> {
        let data: Vec<String> = serde_json::from_str(&data)?;

        Ok(data
            .iter()
            .map(|ip| -> Result<(IpAddr, Peer), PeersFileControllerError> {
                Ok((
                    IpAddr::from_str(ip).inspect_error(|err| warn!("Can't parse ip {}", err))?,
                    Peer::new(ip)?,
                ))
            })
            .flatten()
            .collect())
    }

    pub fn read_file(&self) -> Result<HashMap<IpAddr, Peer>, PeersFileControllerError> {
        let json = fs::read_to_string(&self.file_path)?;

        Self::parse_peer(json)
    }

    pub async fn write_file(
        &self,
        peers: &RwLock<HashMap<IpAddr, Peer>>,
    ) -> Result<(), PeersFileControllerError> {
        if !self.is_changed.load(Ordering::Relaxed) {
            return Ok(());
        };

        let ips: Vec<String> = peers
            .read()
            .await
            .iter()
            .map(|peer| peer.0.to_string())
            .collect();
        let json = serde_json::to_string(&ips)?;

        fs::write(&self.file_path, &json)?;
        self.is_changed.store(false, Ordering::SeqCst);

        info!("Json peers list dumped");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;

    #[test]
    fn test_read_file() {
        let input = "[\"192.168.1.1\", \"192.168.2.1\", \"192.168.3.1\"]".to_string();

        let peers: Vec<String> = PeersFileController::parse_peer(input)
            .expect("A list of peers")
            .keys()
            .map(IpAddr::to_string)
            .collect();

        assert_eq!(3, peers.len());
        assert!(peers.contains(&String::from("192.168.1.1")));
        assert!(peers.contains(&String::from("192.168.2.1")));
        assert!(peers.contains(&String::from("192.168.3.1")));
    }
}
