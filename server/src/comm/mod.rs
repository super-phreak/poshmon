pub mod structs;

use crate::engine::{structs::{Pokemon, GameState, PokeTeam, DataFieldNotFoundError}, data::Data, create_pokemon};

use self::structs::{
    Peer,
    Response,
    Commands,
    Communication, ServerConfig, GameStateModel, PlayerPokemonModel,
};

use std::{
        net::SocketAddr,
        error::Error,
        sync::{Arc, Mutex, RwLock},
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

fn create_pokemon_model(mon: &RwLock<Pokemon>) -> Result<PlayerPokemonModel, Box<dyn Error>> {
    if let Ok(pokemon) = mon.read() {
        Ok(PlayerPokemonModel{
            id: pokemon.base.pokedex,
            nickname: pokemon.nickname.clone(),
            level: pokemon.level,
            hp: pokemon.hp,
            current_hp: pokemon.current_hp,
            attack: pokemon.attack,
            defense: pokemon.defense,
            speed: pokemon.speed,
            special: pokemon.special,
            guid: pokemon.guid.to_string(),
            moves: vec![pokemon.move1.as_ref().map_or(None, |mv| Some(mv.id)),pokemon.move2.as_ref().map_or(None, |mv| Some(mv.id)),pokemon.move3.as_ref().map_or(None, |mv| Some(mv.id)),pokemon.move4.as_ref().map_or(None, |mv| Some(mv.id))],
        })
    } else {
        return Err(DataFieldNotFoundError.into());
    }
}

fn get_team_from_ids(ids: Vec<i64>, data: Data) -> Result<PokeTeam, Box<dyn Error>> {
    let mut team: Vec<Arc<RwLock<Pokemon>>> = Vec::new();
    for id in ids {
        team.push(Arc::new(RwLock::new(create_pokemon(id as u8, data.clone())?)));
    }
    return Ok(PokeTeam::new(team));
}

fn build_pokemodel(team: PokeTeam, player: bool) -> Vec<PlayerPokemonModel> {
    let mut team_model = Vec::new();
    for mon in team.iter() {
        if let Ok(pokemodel) = create_pokemon_model(&mon.clone()) {
            team_model.push(pokemodel);
        }
    }

    return team_model;
}

fn get_gamestate(session_id: & String, move_id: i32, data: Data) -> Result<GameStateModel, Box<dyn Error>> {
    match data.games.lock().unwrap().get(session_id) {
        Some(game) => {
            if let Ok(mut fight) = game.write() {
                fight.fight(data.movedex.get(&(move_id as u8)).unwrap(), data.movedex.get(&(1 as u8)).unwrap());
                Ok(GameStateModel{ player_mon: create_pokemon_model(&fight.active1)?, enemy_mon: create_pokemon_model(&fight.active2)?, fight_message: fight.last_fight.lock().unwrap().unwrap().player1_movestatus })
            } else {
                Err(DataFieldNotFoundError.into())
            }
        },
        None => Err(DataFieldNotFoundError.into()),
    }
}

fn build_game(player1_team: PokeTeam, player2_team: PokeTeam) -> Result<GameState, Box<dyn Error>> {
    return Ok(GameState { player1_team: player1_team.clone(), player2_team: player2_team.clone(), active1: player1_team.get(0).ok_or_else(|| DataFieldNotFoundError)?.clone(), active2: player2_team.get(1).ok_or_else(|| DataFieldNotFoundError)?.clone(), last_fight: Mutex::new(None), player1_ready: RwLock::new(true), player2_ready: RwLock::new(true) });
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
                Commands::CreateGame {  } => Response::Awk { session_id: "dfad".to_string(), cmd_response: "dafd".to_string() }.to_message(),
                Commands::SubmitTeam {session_id, client_id, name, team } => {
                    let team2: Vec<i64> = vec![25,25];
                    if let Ok(game) = build_game(get_team_from_ids(team, data.clone()).ok().unwrap(), get_team_from_ids(team2, data.clone()).ok().unwrap()) {
                        let game = Arc::new(RwLock::new(game));
                        data.games.lock().unwrap().insert(session_id.clone(), game.clone());
                        Response::SubmitTeam { session_id: session_id, client_id: client_id, name: "Josh".to_string(), team: build_pokemodel(game.clone().read().unwrap().player1_team.clone(), true), valid: true }.to_message()
                    } else {
                        Response::Awk { session_id: session_id, cmd_response: "Failure to submit team".to_string() }.to_message()
                    }
                },
                Commands::SendMove { session_id, client_id, pokemon_guid: _, move_id } => Response::BattleResult { game_state: get_gamestate(&session_id, move_id, data.clone()).unwrap(), session_id, client_id }.to_message(),
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