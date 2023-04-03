pub mod structs;

extern crate poshmon_lib;
use poshmon_lib::{
    networking::{Packet, Communication, Datagram, SessionToken}, engine::gen1::{Pokemon, PokemonModel}
};

use crate::engine::{data::Data, structs::DataFieldNotFoundError};

use self::structs::{
    Peer,
    ServerConfig,
};

use std::{
        net::SocketAddr,
        error::Error,
        sync::{Arc, Mutex, RwLock, PoisonError, RwLockReadGuard},
        collections::HashMap,
    };


use tokio::net::{TcpStream};
    
use uuid::Uuid;
use tungstenite::protocol::Message;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};


type PeerMap = Arc<Mutex<HashMap<u128,Peer>>>;

fn new_peer(addr: SocketAddr, name: Option<String>, tx: UnboundedSender<Message>) -> Peer {
    let peer: Peer = Peer {addr, client_id: Uuid::new_v4(), name, tx};
    return peer;
}

fn _create_pokemon_model(mon: &RwLock<Pokemon>, reduced: bool) -> Result<PokemonModel, PoisonError<RwLockReadGuard<Pokemon>>> {
    match mon.read() {
        Ok(mon) => Ok(mon.to_model(reduced)),
        Err(e) => Err(e),
    }
}

// fn get_team_from_ids(ids: Vec<i64>, data: Data) -> Result<PokeTeam, Box<dyn Error>> {
//     let mut team: Vec<Arc<RwLock<Pokemon>>> = Vec::new();
//     for id in ids {
//         team.push(Arc::new(RwLock::new(create_pokemon(id as u8, data.clone())?)));
//     }
//     return Ok(PokeTeam::new(team));
// }

pub async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr, data: Data) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (tx, rx) = unbounded();
    // Insert the write part of this peer to the peer map.
    let client = new_peer(addr, None, tx);
    let session = Uuid::new_v4();
    peer_map.lock().unwrap().insert(session.as_u128(), client);

    let (outgoing, incoming) = ws_stream.split();

    let incoming_msg = incoming.try_for_each(|msg| {
        println!("Received a message from {}: {}", &addr, &msg.to_text().unwrap());
        let unwrapped_msg = match msg.to_text() {
            Ok(msg) => msg.to_owned(),
            Err(e) => format!("errored_out {}", e),
        };
        let packet: Result<Packet, serde_json::Error> = serde_json::from_str(&unwrapped_msg);
        let cmd = match packet {
            Ok(pack) => {
                match data.debug.read() {
                    Ok(debug_map) => {
                        let false_str = "false".to_owned();
                        let token_id = debug_map.get("session_id").unwrap_or(&false_str);
                        match pack.verify(token_id).is_ok() {
                            true => Ok(pack.data),
                            false => Err(DataFieldNotFoundError::new("Datagram")),
                        }
                    },
                    Err(_) => Err(DataFieldNotFoundError::new("Debug Session ID")),
                }
            }
            Err(_) => Err(DataFieldNotFoundError::new("Signature")),
        };

        let msg_out;
        let token = SessionToken::new("testuser".to_owned());
        if let Ok(cmd_in) = cmd  {
            msg_out = match cmd_in {
                Datagram::JoinGame { username: _, game_id: _ } => Packet::new(token, Datagram::Awk { session_id: "dfad".to_string(), cmd_response: "dafd".to_string() }),
                Datagram::SubmitTeam {session_id: _, client_id: _, name: _, team: _ } => Packet::new(token, Datagram::Awk { session_id: "test".to_string(), cmd_response: "test".to_string() }),
                Datagram::SendMove { session_id: _, client_id: _, pokemon_guid: _, move_id: _ } => Packet::new(token, Datagram::Awk { session_id: "test".to_string(), cmd_response: "test".to_string() }), //{ /*gamestate: get_gamestate(&"1234".to_string(),move_id,data.clone()).ok().unwrap(),*/ session_id, client_id }),
                Datagram::GetTeam { session_id: _, client_id: _, name: _ } => Packet::new(token, Datagram::Awk { session_id: "test".to_string(), cmd_response: "test".to_string() }),
                Datagram::Awk { session_id: _, cmd_response: _ } => Packet::new(token, Datagram::Awk { session_id: "test".to_string(), cmd_response: "test".to_string() }),
                //Datagram::BattleResult { client_id, session_id } => todo!(),
                //Commands::Chat { client_id, recipient, chat_msg } => Response::
                //_ => (Message::from(format!("Player Invalid CMD")), Message::from(format!("You sent invalid cmd"))),
            };
        } else if msg.is_empty() {
            msg_out = Packet::new(token, Datagram::Awk { session_id: "ping".to_string(), cmd_response: "ping".to_string() })
        //     msg_out = Message::from(format!("{{\"action\": \"Ping\"}}"));
        } else {
            msg_out = Packet::new(token, Datagram::Awk { session_id: "err".to_string(), cmd_response: "err".to_string() })
        //     msg_out = Message::from(format!("{{\"action\": \"ERR\"}}"));
        };

        let peers = peer_map.lock().unwrap();
        
        // if let Some(to_self) = peers.iter().find().map(|(_, ws_sink)| ws_sink) {
        //     to_self.unbounded_send(player_msg).unwrap();
        // }
        // We want to broadcast the message to everyone except ourselves.
        // let broadcast_recipients =
        //     peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);

        // let recipients =
        //     peers.iter().filter(|(_, peerd)| peerd.addr != addr).map(|(_, peerd)| peerd);

        // for recp in recipients {
        //     recp.tx.unbounded_send(msg_out.clone()).unwrap();
        // }
        for p in peers.iter().map(|(_, pd)| pd) {
            p.tx.unbounded_send(msg_out.to_message().clone()).unwrap();
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(incoming_msg, receive_from_others);
    future::select(incoming_msg, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&session.as_u128());
}

pub fn create_server_config(port: Option<u16>) -> Result<ServerConfig, Box<dyn Error>> {
    let config = ServerConfig{
        ip: local_ipaddress::get().unwrap_or("127.0.0.1".to_string()),
        port: port.unwrap_or(8080),
        peers: PeerMap::new(Mutex::new(HashMap::new())),
    };
    return Ok(config);
}