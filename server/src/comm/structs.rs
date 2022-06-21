use std::{
    //    env,
    //    io::Error as IoError,
        net::SocketAddr,
        hash::Hash,
    };
    
use futures_channel::mpsc::{UnboundedSender};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use tungstenite::protocol::Message;

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum Commands {
    Login {},
    SubmitTeam {client_id: String},
    Chat {client_id: String, recipient: String, chat_msg: String}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum Response {
    Login {client_id: String, auth: bool},
    SubmitTeam {client_id: String}
}

pub trait Communication {
    fn to_json(&self) -> String where
        Self: Serialize {
        match serde_json::to_string(&self) {
            Ok(resp) => return resp,
            Err(_) => return String::from("{\"err\": 500}")
        }
    }
    fn to_message(&self) -> Message where 
        Self: Serialize {
            return Message::from(self.to_json());
    }
}

impl Communication for Response {}