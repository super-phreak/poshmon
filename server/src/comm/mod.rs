pub mod structs;

use self::structs::{
    Peer,
    Response,
    Commands,
    Communication, ServerConfig,
};

use std::{
        env,
        io::Error as IoError,
        net::SocketAddr,
        error::Error,
        sync::{Arc, Mutex},
        collections::{HashMap, HashSet},
    };

use tokio::net::{TcpListener, TcpStream};
    
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use tungstenite::protocol::Message;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};


type PeerMap = Arc<Mutex<HashMap<u128,Peer>>>;

fn new_peer(addr: SocketAddr, name: Option<String>, tx: UnboundedSender<Message>) -> Peer {
    let peer: Peer = Peer {addr, client_id: Uuid::new_v4(), name, tx};
    return peer;
}

pub async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (tx, rx) = unbounded();
    // Insert the write part of this peer to the peer map.
    let client = new_peer(addr, None, tx);
    let session = Uuid::new_v4().as_u128();
    peer_map.lock().unwrap().insert(session, client);

    let (outgoing, incoming) = ws_stream.split();

    let incoming_msg = incoming.try_for_each(|msg| {
        println!("Received a message from {}: {}", &addr, &msg.to_text().unwrap());
        //let client_id = &peer.client_id.as_u128();
        let cmd: Result<Commands, _> = serde_json::from_str(msg.to_text().unwrap());
        let msg_out: Message;
        let player_msg: Message;
        if let Ok(data) = cmd  {
            let (temp_out, temp_msg) = match data {
                Commands::Login {} => (Message::from(format!("Player {} joined", "")), Response::Login{client_id: "jfqsdcja".to_string(), auth: true}.to_message()),
                Commands::SubmitTeam {client_id} => (Message::from(format!("Player {} submitted team", client_id)), Message::from(format!("You submitted team"))),
                Commands::Chat { client_id, recipient, chat_msg } => (Message::from(format!("")),Message::from(format!("")))
                //_ => (Message::from(format!("Player Invalid CMD")), Message::from(format!("You sent invalid cmd"))),
            };
            msg_out = temp_out;
            player_msg = temp_msg;
        } else if msg.is_empty() {
            msg_out = Message::from(format!("Player Pinged"));
            player_msg = Message::from(format!("{{\"action\": \"Ping\"}}"));
        } else {
            msg_out = Message::from(format!("ERR"));
            player_msg = Message::from(format!("Invaild Command: {}", msg.to_text().unwrap()));
        };

        let peers = peer_map.lock().unwrap();
        
        if let Some(client) = peers.get(&session) {
            client.tx.unbounded_send(player_msg.clone()).unwrap();
        }
        // if let Some(to_self) = peers.iter().find().map(|(_, ws_sink)| ws_sink) {
        //     to_self.unbounded_send(player_msg).unwrap();
        // }
        // We want to broadcast the message to everyone except ourselves.
        // let broadcast_recipients =
        //     peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);

        let recipients =
            peers.iter().filter(|(_, peerd)| peerd.addr != addr).map(|(_, peerd)| peerd);

        for recp in recipients {
            recp.tx.unbounded_send(msg_out.clone()).unwrap();
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(incoming_msg, receive_from_others);
    future::select(incoming_msg, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&session);
}

pub fn create_server_config(port: Option<u16>) -> Result<ServerConfig, Box<dyn Error>> {
    let config = ServerConfig{
        ip: local_ipaddress::get().ok_or("127.0.0.1")?,
        port: port.ok_or(8080).unwrap(),
        peers: PeerMap::new(Mutex::new(HashMap::new())),
    };
    return Ok(config);
}