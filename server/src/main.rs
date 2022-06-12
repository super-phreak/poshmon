mod engine;
mod comm;

use comm::Peer;
use std::fs::File;
use engine::structs::{Pokemon, PokeType};
use rand::Rng;
use rand::prelude::SliceRandom;
//use tokio::{io as tokio_io, task};
use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use std::{
    collections::HashMap,
//    env,
//    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use uuid::Uuid;
use serde::{Serialize, Deserialize};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<Peer, Tx>>>;



fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug)]
struct ServerConfig {
    ip: String,
    port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename_all = "snake_case")]
enum Commands {
    Login {},
    SubmitTeam {client_id: String},
    Chat {client_id: String, recipient: String, chat_msg: String}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename_all = "snake_case")]
enum Response {
    Login {client_id: String, auth: bool},
    SubmitTeam {client_id: String}
}

trait Communication {
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

fn create_server_config() -> Result<ServerConfig, Box<dyn Error>> {
    let config = ServerConfig{
        ip: local_ipaddress::get().ok_or("127.0.0.1")?,
        port: 8080
    };
    return Ok(config);
}

fn new_peer(addr: SocketAddr) -> Peer {
    let peer: Peer = Peer { addr, client_id: Uuid::new_v4() };
    return peer;
}

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (tx, rx) = unbounded();
    // Insert the write part of this peer to the peer map.
    let peer = new_peer(addr);
    peer_map.lock().unwrap().insert(peer, tx);

    let (outgoing, incoming) = ws_stream.split();

    let incoming_msg = incoming.try_for_each(|msg| {
        println!("Received a message from {}: {}", addr, msg.to_text().unwrap());
        let cmd: Result<Commands, _> = serde_json::from_str(msg.to_text().unwrap());
        let msg_out: Message;
        let player_msg: Message;
        if let Ok(data) = cmd  {
            let (temp_out, temp_msg) = match data {
                Commands::Login {} => (Message::from(format!("Player {} joined", &peer.client_id)), Response::Login{client_id: peer.client_id.to_string(), auth: true}.to_message()),
                Commands::SubmitTeam {client_id} => (Message::from(format!("Player {} submitted team", client_id)), Message::from(format!("You submitted team"))),
                Commands::Chat { client_id, recipient, chat_msg } => (Message::from(format!("")),Message::from(format!("")))
                //_ => (Message::from(format!("Player Invalid CMD")), Message::from(format!("You sent invalid cmd"))),
            };
            msg_out = temp_out;
            player_msg = temp_msg;
        } else if msg.is_empty() {
            msg_out = Message::from(format!("Player Pinged"));
            player_msg = Message::from(format!("You Pinged"));
        } else {
            msg_out = Message::from(format!("ERR"));
            player_msg = Message::from(format!("Invaild Command: {}", msg.to_text().unwrap()));
        };
        

        let peers = peer_map.lock().unwrap();
        
        if let Some(to_self) = peers.iter().find(|(self_addr, _)| self_addr.addr == addr).map(|(_, ws_sink)| ws_sink) {
            to_self.unbounded_send(player_msg).unwrap();
        }
        // We want to broadcast the message to everyone except ourselves.
        // let broadcast_recipients =
        //     peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);

        let recipients =
            peers.iter().filter(|(peer, _)| peer.addr != addr).map(|(_, ws_sink)| ws_sink);

        for recp in recipients {
            recp.unbounded_send(msg_out.clone()).unwrap();
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(incoming_msg, receive_from_others);
    future::select(incoming_msg, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&peer);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let mut pokedex: Vec<Pokemon> = Vec::new();
    let mut typedex: Vec<PokeType> = Vec::new();
    let mut rng = rand::thread_rng();
    let engine_conf = File::open("../data/engine.json").expect("Unable to read file");
    let pokedex_file = File::open("../data/pokedex.json").expect("unable to open pokedex");
    let engine_json: serde_json::Value = serde_json::from_reader(engine_conf).expect("JSON was not well-formatted");
    let pokedex_json: serde_json::Value = serde_json::from_reader(pokedex_file).expect("JSON was not well-formatted");
    let level: i32 = 100;

    for poketypes in engine_json["types"].as_array().unwrap() {
        if let Some(new_type) = engine::build_type(poketypes) {
            typedex.push(new_type);
        }
    }

    for pokemon_json in pokedex_json.as_array().unwrap() {
        if let Some(new_mon) = engine::build_pokemon(pokemon_json,pokemon_json["name"].as_str().unwrap(),level,&typedex,rng.gen_range(0..u16::MAX) as i32) {
            pokedex.push(new_mon);
        }
    }
    //println!("Types: {:#?}", &typedex_vec.into_iter().find(|x| x.index == 23));
    assert_eq!(pokedex.len(), 151, "Pokedex length should be {} but {} was found", 151, pokedex.len());

    println!("{:#?}", pokedex.choose(&mut rng).unwrap());
    let server_configs = create_server_config()?;
    println!("{:#?}", server_configs);

    let addr = format!("{}:{}", server_configs.ip, server_configs.port);
    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }

    Ok(())

}
