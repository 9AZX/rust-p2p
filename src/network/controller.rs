use std::io;
use std::net::{IpAddr};
use tokio::net::TcpStream;
use crate::network::file::PeersFileController;
use crate::network::peer::Peer;

#[derive(Default, Clone, Copy, Debug)]
pub struct NetworkController;

impl NetworkController {
    pub async fn new(
        peers_file: &str,
        listen_port: u16,
        target_outgoing_connections: &mut Vec<Peer>,
        max_incoming_connections: usize,
        max_simultaneous_outgoing_connection_attempts: usize,
        max_simultaneous_incoming_connection_attempts: usize,
        max_idle_peers: usize,
        max_banned_peers: usize,
        peer_file_dump_interval_seconds: usize,
    ) -> Result<Self, io::Error> {
        target_outgoing_connections.append(&mut PeersFileController::read(peers_file));
        Ok(Self)
    }

    pub async fn wait_event(&self) -> Result<NetworkControllerEvent, io::Error> {
        todo!()
    }
}

pub enum NetworkControllerEvent {
    CandidateConnection {
        ip: IpAddr,
        socket: TcpStream,
        is_outgoing: bool,
    },
}
