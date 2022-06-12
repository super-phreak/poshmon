use std::{
//    env,
//    io::Error as IoError,
    net::SocketAddr,
};

use uuid::Uuid;

#[derive(Eq, Hash, Copy, Clone)]
pub struct Peer {
    pub addr: SocketAddr,
    pub client_id: Uuid,
}
impl PartialEq for Peer {
    fn eq(&self, other: &Self) -> bool {
        return self.addr == other.addr;
    }
}