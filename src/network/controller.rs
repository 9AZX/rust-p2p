use std::collections::HashMap;
use std::io;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;

use displaydoc::Display;
use log::{error, info, warn};
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{mpsc, RwLock};
use tokio::task;

use crate::network::controller::NetworkControllerEvent::CandidateConnection;
use crate::network::file::{PeersFileController, PeersFileControllerError};
use crate::network::message::ChannelMessage;
use crate::network::message::ChannelMessage::Connection;
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
    /// Error sending a message in the channel
    ChannelError { peer_ip: IpAddr },
    /// The channel is closed
    ClosedChanel,
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
    connect_to_peers_handle: task::JoinHandle<()>,
    channel_sender: UnboundedSender<ChannelMessage>,
    channel_receiver: UnboundedReceiver<ChannelMessage>,
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

        let peers_clone_file_controller = peers.clone();
        let peers_clone_task_connect = peers.clone();

        // Create the file dumper worker
        let file_controller_clone = file_controller.clone();
        tokio::spawn(async move {
            info!("Starting file worker");
            let mut interval =
                tokio::time::interval(Duration::from_secs(peer_file_dump_interval_seconds));
            loop {
                interval.tick().await;
                let _ = file_controller_clone.write_file(peers_clone_file_controller.as_ref());
            }
        });

        let (channel_sender, mut channel_receiver) = mpsc::unbounded_channel::<ChannelMessage>();

        // Create task for connecting to peers
        let channel_sender_connect_peers = channel_sender.clone();
        let connect_to_peers_handle = task::spawn(async move {
            Self::connect_to_peers(
                peers_clone_task_connect,
                listen_port,
                channel_sender_connect_peers,
            )
            .await;
        });

        // Create task for listening new peers
        let channel_listen_connect_peers = channel_sender.clone();
        let listen_connecting_peer = task::spawn(async move {
            Self::listen_new_peers(listen_port, channel_listen_connect_peers).await;
        });

        Ok(Self {
            file_controller,
            peers,
            target_outgoing_connections,
            listen_port,
            connect_to_peers_handle,
            channel_sender,
            channel_receiver,
        })
    }

    pub async fn add_peer(
        &mut self,
        ip: String,
        socket: Option<TcpStream>,
    ) -> Result<(), NetworkControllerError> {
        let mut peer = Peer::new(&ip)?;
        peer.socket = socket;
        self.peers.write().await.insert(*peer.ip(), peer);
        self.file_controller.changed();

        Ok(())
    }

    pub fn remove_peer(&self) {
        todo!()
    }

    pub async fn wait_event(&mut self) -> Result<NetworkControllerEvent, NetworkControllerError> {
        if let Some(event) = self.channel_receiver.recv().await {
            match event {
                Connection {
                    ip,
                    socket,
                    is_outgoing,
                } => Ok(CandidateConnection {
                    ip: ip.to_string(),
                    socket: socket,
                    is_outgoing,
                }),
                _ => todo!(),
            }
        } else {
            Err(NetworkControllerError::ClosedChanel)
        }
    }

    pub async fn listen_new_peers(
        listen_port: u16,
        sender: UnboundedSender<ChannelMessage>,
    ) -> Result<(), NetworkControllerError> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", listen_port)).await?;

        loop {
            let (socket, addr) = listener.accept().await?;

            sender
                .send(Connection {
                    ip: addr.ip(),
                    socket: socket,
                    is_outgoing: false,
                })
                .map_err(|err| NetworkControllerError::ChannelError { peer_ip: addr.ip() })?;
        }
    }

    pub async fn connect_to_peers(
        peers: Arc<RwLock<HashMap<IpAddr, Peer>>>,
        listen_port: u16,
        sender: UnboundedSender<ChannelMessage>,
    ) -> Result<(), NetworkControllerError> {
        for peer in peers.write().await.iter_mut() {
            peer.1.connecting();
            if let Ok(socket) = TcpStream::connect(format!(
                "{}:{}",
                peer.0.to_string(),
                listen_port.to_string()
            ))
            .await
            {
                sender
                    .send(Connection {
                        ip: *peer.0,
                        socket,
                        is_outgoing: true,
                    })
                    .map_err(|err| NetworkControllerError::ChannelError { peer_ip: *peer.0 })?;
            } else {
                peer.1.idle();
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

    pub fn feedback_peer_failed(&self, ip: &IpAddr) {
        todo!()
    }

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
