use std::{fs, io};
use std::collections::HashMap;
use std::net::{AddrParseError, IpAddr};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;
use crate::network::peer::{Peer};
use thiserror::Error;
use displaydoc::Display;
use log::{error};
use crate::error_logger::InspectErr;

struct FileWorker;

#[derive(Display, Error, Debug)]
pub enum PeersFileControllerError {
    /// Unable to read peers json file
    Io(#[from] io::Error),
    /// JSON does not have correct format.
    Serialization(#[from] serde_json::Error),
    /// Can't parse IP Address: {0}
    IpAddressFormat(#[from] AddrParseError)
}

#[derive(Default)]
pub struct PeersFileController {
    file_path: String,
    is_changed: bool
}

impl PeersFileController {
    pub fn new(file: &str) -> PeersFileController {
        PeersFileController { file_path: file.to_string(), is_changed: false }
    }

    fn parse_peer(data: String) -> Result<HashMap<IpAddr, Peer>, PeersFileControllerError> {
        let data: Vec<String> = serde_json::from_str(&data)?;


        Ok(data.iter().map(|ip| -> Result<(IpAddr, Peer), PeersFileControllerError> {
            Ok((IpAddr::from_str(ip).inspect_error(|err| error!("Can't parse ip {}", err))?, Peer::new(ip)))
        }).flatten().collect())
    }

    pub fn read_file(&self) -> Result<HashMap<IpAddr, Peer>, PeersFileControllerError> {
        let json = fs::read_to_string(&self.file_path)?;

        Self::parse_peer(json)
    }

    pub fn write_file(&self, peers: &Vec<Peer>) -> Result<(), io::Error> {
        let ips: Vec<String> = peers.iter().map(|peer| peer.ip().to_string()).collect();

        let json = serde_json::to_string(&ips)?;

        fs::write(&self.file_path, &json)

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;

    #[test]
    fn test_read_file() {
        let input = "[\"192.168.1.1\", \"192.168.4322.2\", \"192.168.2.1\", \"192.168.1.1\"]".to_string();

        let peers: Vec<String> = PeersFileController::parse_peer(input).expect("A list of peers").keys().map(IpAddr::to_string).collect();

        assert_eq!(peers,
            ["192.168.1.1", "192.168.2.1"].as_slice()
        );
    }
}
