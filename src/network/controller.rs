use std::collections::HashMap;
use std::io;
use std::net::IpAddr;

use tokio::net::TcpStream;
use tokio::time::{sleep};
use thiserror::Error;
use displaydoc::Display;

use crate::network::file::{PeersFileController, PeersFileControllerError};
use crate::network::peer::Peer;
use crate::error_logger::InspectErr;

#[derive(Display, Error, Debug)]
pub enum NetworkControllerError {
    /// Error with the file controller: {0}
    FileController(#[from] PeersFileControllerError),
}

pub struct NetworkController {
    file_controller: PeersFileController,
    peers: HashMap<IpAddr, Peer>
}

impl NetworkController {
    pub async fn new(
        peers_file: &str,
        listen_port: u16,
        target_outgoing_connections: HashMap<IpAddr, Peer>,
        max_incoming_connections: usize,
        max_simultaneous_outgoing_connection_attempts: usize,
        max_simultaneous_incoming_connection_attempts: usize,
        max_idle_peers: usize,
        max_banned_peers: usize,
        peer_file_dump_interval_seconds: u64,
    ) -> Result<Self, io::Error> {
        let mut network_controller = Self { file_controller: PeersFileController::new(peers_file), peers: target_outgoing_connections};

        // Read json and create peers
      //  network_controller.peers.insert(&mut network_controller.file_controller.read_file());

        // Create the file dumper worker

        Ok(network_controller)
    }

    pub async fn add_peer(&self) {}

    pub async fn remove_peer(&self) {}

    pub async fn wait_event(&self) -> Result<NetworkControllerEvent, io::Error> {
        todo!()
    }

    async fn connect_to_peers(&mut self) {
        for peer in &mut self.peers {
        }
    }
}

pub enum NetworkControllerEvent {
    CandidateConnection {
        ip: IpAddr,
        socket: TcpStream,
        is_outgoing: bool,
    },
}
