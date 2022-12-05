use std::io;
use std::net::{IpAddr};
use tokio::net::TcpStream;
use crate::network::file::PeersFileController;
use crate::network::peer::Peer;

pub struct NetworkController {
    pub file_controller: PeersFileController
}

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
        let mut network_controller = NetworkController { file_controller: PeersFileController::new(peers_file) };

        target_outgoing_connections.append(&mut network_controller.file_controller.read_file());

        network_controller.file_controller.write_file(target_outgoing_connections);

        Ok(network_controller)
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
