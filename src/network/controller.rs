use std::io;
use std::net::{IpAddr, TcpStream};

#[derive(Default, Clone, Copy, Debug)]
pub struct NetworkController;

impl NetworkController {
    pub async fn new() -> Result<Self, io::Error> {
        Ok(Self)
    }

    pub async fn wait_event(&self) -> Result<NetworkControllerEvent, io::Error> {
        todo!()
    }
}

pub enum NetworkControllerEvent {
    CandidateConnection { ip: IpAddr, socket: TcpStream, is_outgoing: bool }
}