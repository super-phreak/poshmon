mod engine;
mod comm;

use comm::create_server_config;
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

use comm::handle_connection;

type Tx = UnboundedSender<Message>;




fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
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
    let server_configs = create_server_config(Some(8080))?;
    println!("{:#?}", server_configs);

    let addr = format!("{}:{}", server_configs.ip, server_configs.port);
    let state = server_configs.peers;

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
