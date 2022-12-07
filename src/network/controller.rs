use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::io;
use std::net::IpAddr;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use displaydoc::Display;
use log::{error, info, warn};
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};

use crate::error_logger::InspectErr;
use crate::network::controller::NetworkControllerEvent::CandidateConnection;
use crate::network::file::{PeersFileController, PeersFileControllerError};
use crate::network::peer::{Peer, PeerError};

#[derive(Display, Error, Debug)]
pub enum NetworkControllerError {
    /// Error with the file controller: {0}
    FileController(#[from] PeersFileControllerError),
    /// Io error: {0}
    Io(#[from] io::Error),
    /// Impossible to acquire lock over a RwLock
    RwLockPoisoned,
    /// Cannot manage peer {0}
    PeerError(#[from] PeerError),
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
    listen_port: u16,
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
                    .inspect_error(|err| error!("Error while writing file: {err}"));
            }
        });

        // Create the controller
        let mut controller = Self {
            file_controller,
            peers,
            target_outgoing_connections,
            listen_port,
        };

        // Try to connect to know peers
        controller.connect_to_peers().await?;

        Ok(controller)
    }

    pub fn add_peer(
        &mut self,
        ip: String,
        socket: Option<TcpStream>,
    ) -> Result<(), NetworkControllerError> {
        let mut peer = Peer::new(&ip)?;
        peer.socket = socket;
        if let Ok(mut peers) = self.peers.write() {
            peers.insert(*peer.ip(), peer);
            self.file_controller.changed();
        } else {
            error!("Peer list is poisoned and won't get refresh");
        }

        Ok(())
    }

    pub fn remove_peer(&self) {
        todo!()
    }

    pub async fn wait_event(&self) -> Result<NetworkControllerEvent, io::Error> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.listen_port)).await?;

        loop {
            let (socket, addr) = listener.accept().await?;

            return Ok(CandidateConnection {
                ip: addr.ip().to_string(),
                socket,
                is_outgoing: false,
            });
        }
    }

    pub async fn connect_to_peers(&mut self) -> Result<(), NetworkControllerError> {
        for peer in self
            .peers
            .write()
            .map_err(|_| NetworkControllerError::RwLockPoisoned)?
            .iter_mut()
        {
            peer.1.connecting();
            if let Ok(socket) = TcpStream::connect(format!(
                "{}:{}",
                peer.0.to_string(),
                self.listen_port.to_string()
            ))
            .await
            {
                peer.1.socket = Some(socket);
                info!(
                    "Connect to {}:{}",
                    peer.0.to_string(),
                    self.listen_port.to_string()
                );
            } else {
                peer.1.idle();
                warn!(
                    "Cannot connect to {}:{}",
                    peer.0.to_string(),
                    self.listen_port.to_string()
                );
            }
        }

        Ok(())
    }

    pub fn feedback_peer_alive(&self, ip: &IpAddr) {
        todo!()
    }

    pub fn feedback_peer_banned(&self, ip: &IpAddr) {
        todo!()
    }

    pub fn feedback_peer_failed(&self, ip: &IpAddr) { todo!() }

    pub fn feedback_peer_closed(&self, ip: &IpAddr) {
        todo!()
    }

    pub fn feedback_peer_list(&self) {
        todo!()
    }

    pub fn get_good_peer_ips(&self) {
        todo!()
    }
}

pub enum NetworkControllerEvent {
    CandidateConnection {
        ip: String,
        socket: TcpStream,
        is_outgoing: bool,
    },
}
