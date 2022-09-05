mod engine;
mod comm;

use comm::create_server_config;
use std::fs::File;
use rand::Rng;
//use tokio::{io as tokio_io, task};
use tokio::net::{TcpListener};
use std::error::Error;
use std::{
    collections::HashMap,
};

use comm::handle_connection;

use crate::engine::init_engine;

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    //let pokedex = Pokedex::new(Mutex::new(HashMap::new()));
    //let mut typedex = 
    let mut rng = rand::thread_rng();
    let engine_conf = File::open("../data/engine.json").expect("Unable to read file");
    let pokedex_file = File::open("../data/pokedex.json").expect("unable to open pokedex");
    let movedex_file = File::open("../data/movedex.json").expect("unable to open movedex");
    let words_file = File::open("../data/gamenames.txt").expect("unable to open wordlist");
    let engine_json: serde_json::Value = serde_json::from_reader(engine_conf).expect("JSON was not well-formatted");
    let pokedex_json: serde_json::Value = serde_json::from_reader(pokedex_file).expect("JSON was not well-formatted");
    let movedex_json: serde_json::Value = serde_json::from_reader(movedex_file).expect("JSON was not well-formatted");

    let mut data: HashMap<&str, serde_json::Value> = HashMap::new();
    data.insert("conf", engine_json);
    data.insert("pokemon", pokedex_json);
    data.insert("moves", movedex_json);

    let engine = init_engine(data, words_file);
    //println!("Types: {:#?}", &typedex_vec.into_iter().find(|x| x.index == 23));
    assert_eq!(engine.pokedex.len(), 151, "Pokedex length should be {} but {} was found", 151, engine.pokedex.len());

    println!("{:#?}", engine.pokedex.get(&(rng.gen_range(1..=engine.pokedex.len()) as u8)).unwrap());
    
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
        tokio::spawn(handle_connection(state.clone(), stream, addr, engine.clone()));
    }

    Ok(())

}
