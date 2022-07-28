pub mod structs;

use crate::engine::{structs::{Pokemon}, data::Data, create_pokemon};

use self::structs::{
    Peer,
    Response,
    Commands,
    Communication, ServerConfig, PokemonModel,
};

use std::{
        net::SocketAddr,
        error::Error,
        sync::{Arc, Mutex},
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

fn create_pokemon_model(pokemon: Pokemon) -> Result<PokemonModel, Box<dyn Error>> {
    Ok(PokemonModel{
        id: pokemon.base.pokedex,
        nickname: pokemon.nickname,
        level: pokemon.level,
        hp: pokemon.hp,
        current_hp: pokemon.hp,
        attack: pokemon.attack,
        defense: pokemon.defense,
        speed: pokemon.speed,
        special: pokemon.special,
        guid: pokemon.guid.to_string(),
        moves: vec![pokemon.move1.map_or(None, |mv| Some(mv.id)),pokemon.move2.map_or(None, |mv| Some(mv.id)),pokemon.move3.map_or(None, |mv| Some(mv.id)),pokemon.move4.map_or(None, |mv| Some(mv.id))],
    })
}

fn get_team_from_ids(ids: Vec<i64>, data: Data) -> Result<Vec<PokemonModel>, Box<dyn Error>> {
    let mut team: Vec<PokemonModel> = Vec::new();
    for id in ids {
        if let Ok(pokemon) = create_pokemon_model(create_pokemon(id as u8, data.clone())?) {
            team.push(pokemon);
        }
    }
    return Ok(team);
}

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
        let values: Result<Commands, serde_json::Error> = serde_json::from_str(msg.to_text().unwrap());
        match values {
            Ok(v) => println!("Deserialzed value to {:#?}" , v),
            Err(e) => println!("Errored {}", e),
        }
        let cmd: Result<Commands, _> = serde_json::from_str(msg.to_text().unwrap());
        let msg_out: Message;
        if let Ok(cmd_in) = cmd  {
            msg_out = match cmd_in {
                Commands::Login {} => Response::Login{client_id: "jfqsdcja".to_string(), session_id: session.to_string(), auth: true}.to_message(),
                Commands::SubmitTeam {session_id, client_id, name, team } => Response::SubmitTeam {session_id, client_id, name, team: get_team_from_ids(team, data.clone()).ok().unwrap(), valid: true }.to_message(),
                Commands::SendMove { session_id, client_id: _, pokemon_guid: _, move_id: _ } => Response::Awk { session_id, cmd_response: "SendMove".to_string() }.to_message(),
                //Commands::Chat { client_id, recipient, chat_msg } => Response::
                //_ => (Message::from(format!("Player Invalid CMD")), Message::from(format!("You sent invalid cmd"))),
            };
        } else if msg.is_empty() {
            msg_out = Message::from(format!("{{\"action\": \"Ping\"}}"));
        } else {

            msg_out = Message::from(format!("{{\"action\": \"ERR\"}}"));
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
            p.tx.unbounded_send(msg_out.clone()).unwrap();
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
        ip: local_ipaddress::get().ok_or("127.0.0.1")?,
        port: port.ok_or(8080).unwrap(),
        peers: PeerMap::new(Mutex::new(HashMap::new())),
    };
    return Ok(config);
}