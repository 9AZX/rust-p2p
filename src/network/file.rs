use std::fs;
use std::net::IpAddr;
use std::str::FromStr;
use crate::network::peer::{Peer};

pub struct PeersFileController;

impl PeersFileController {
    pub fn read(file: &str) -> Vec<Peer> {
        let json = fs::read_to_string(file)
            .expect("Unable to read peers json file");

        let data: Vec<String> = serde_json::from_str(&json)
            .expect("JSON does not have correct format.");

        data.iter().map(|ip| Peer::new(ip)).collect()
    }

    pub fn write(peers: &Vec<Peer>) {
    }
}