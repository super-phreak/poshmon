use std::{
    //    env,
    //    io::Error as IoError,
        net::SocketAddr,
        hash::Hash,
    };
    
use futures_channel::mpsc::UnboundedSender;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use tungstenite::protocol::Message;

use crate::engine::structs::MoveStatus;

use super::PeerMap;

pub type Tx = UnboundedSender<Message>;

#[derive(Debug)]
pub struct Peer {
    pub addr: SocketAddr,
    pub client_id: Uuid,
    pub name: Option<String>,
    pub tx: Tx,
}

impl PartialEq for Peer {
    fn eq(&self, other: &Self) -> bool {
        return self.client_id == other.client_id;
    }
}

impl PartialEq<std::net::SocketAddr> for Peer {
    fn eq(&self, other: &SocketAddr) -> bool {
        return self.addr == *other;
    }
}

impl Hash for Peer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.client_id.hash(state);
    }
}

impl Eq for Peer {}

#[derive(Debug)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
    pub peers: PeerMap,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerPokemonModel {
    pub id: u8,
    pub nickname: String,
    pub level: i32,
    pub hp: i32,
    pub current_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub special: i32,
    pub guid: String,
    pub moves: Vec<Option<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReducedPokemonModel {
    pub id: u8,
    pub nickname: String,
    pub level: i32,
    pub current_hp: i32,
    pub guid: String,
}



#[derive(Debug, Serialize)]
pub struct GameStateModel {
    pub(crate) player_mon: PlayerPokemonModel,
    pub(crate) enemy_mon: PlayerPokemonModel,

    pub(crate) fight_message: MoveStatus,
}

pub trait Payload {

}