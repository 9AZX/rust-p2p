use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::io;
use std::net::IpAddr;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use displaydoc::Display;
use log::{error, info};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio::time::sleep;

use crate::error_logger::InspectErr;
use crate::network::file::{PeersFileController, PeersFileControllerError};
use crate::network::peer::Peer;

#[derive(Display, Error, Debug)]
pub enum NetworkControllerError {
    /// Error with the file controller: {0}
    FileController(#[from] PeersFileControllerError),
    /// Io error: {0}
    Io(#[from] io::Error),
}

impl From<NetworkControllerError> for io::Error {
    fn from(err: NetworkControllerError) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

pub struct NetworkController {
    file_controller: Arc<PeersFileController>,
    peers: Arc<RwLock<HashMap<IpAddr, Peer>>>,
    target_outgoing_connections: HashMap<IpAddr, Peer>,
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
    ) -> Result<Self, NetworkControllerError> {
        let file_controller = Arc::new(PeersFileController::new(peers_file));

        // Read json and create peers
        let peer_list = file_controller.read_file()?;
        let peers: Arc<RwLock<HashMap<IpAddr, Peer>>> = Arc::new(RwLock::new(peer_list));

        // Create the file dumper worker
        let peers_clone = peers.clone();
        let file_controller_clone = file_controller.clone();
        tokio::spawn(async move {
            info!("Starting file worker");
            let mut interval =
                tokio::time::interval(Duration::from_secs(peer_file_dump_interval_seconds));
            loop {
                interval.tick().await;
                let _ = file_controller_clone
                    .write_file(peers_clone.as_ref())
                    .inspect_error(|err| error!("Error while writting file: {err}"));
            }
        });

        Ok(Self {
            file_controller,
            peers,
            target_outgoing_connections,
        })
    }

    pub fn add_peer(&mut self, peer: Peer) {
        if let Ok(mut peers) = self.peers.write() {
            peers.insert(*peer.ip(), peer);
            self.file_controller.changed();
        } else {
            error!("Peer list is poisoned and won't get refresh");
        }
    }

    pub async fn remove_peer(&self) {}

    pub async fn wait_event(&self) -> Result<NetworkControllerEvent, io::Error> {
        todo!()
    }

    async fn connect_to_peers(&mut self) {
        for peer in self.peers.write().unwrap().iter_mut() {
            peer.1.socket = TcpStream::connect(peer.0.to_string()).await.ok();
        }
    }
}

pub enum NetworkControllerEvent {
    CandidateConnection {
        ip: String,
        socket: TcpStream,
        is_outgoing: bool,
    },
}
