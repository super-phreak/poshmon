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

use dotenv::dotenv;

use comm::handle_connection;

use crate::engine::init_engine;

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    match dotenv() {
        Ok(_) => {
            println!("Environment found, running with keys in file.")
        },
        Err(_) => println!("No environment found, running defaults."),
    }

    //let pokedex = Pokedex::new(Mutex::new(HashMap::new()));
    //let mut typedex = 
    let mut rng = rand::thread_rng();
    let data_root = std::env::var("DATA_ROOT").expect("Data folder must be set.");
    let engine_conf = File::open(format!("{}{}",data_root,"engine.json")).expect("Unable to read file");
    let pokedex_file = File::open(format!("{}{}",data_root,"pokedex.json")).expect("unable to open pokedex");
    let movedex_file = File::open(format!("{}{}",data_root,"movedex.json")).expect("unable to open movedex");
    let words_file = File::open(format!("{}{}",data_root,"gamenames.txt")).expect("unable to open wordlist");
    let engine_json: serde_json::Value = serde_json::from_reader(engine_conf).expect("Engine JSON was not well-formatted");
    let pokedex_json: serde_json::Value = serde_json::from_reader(pokedex_file).expect("Pokedex JSON was not well-formatted");
    let movedex_json: serde_json::Value = serde_json::from_reader(movedex_file).expect("Move JSON was not well-formatted");

    //comm::auth::init_db()?;
    // let now = Instant::now();
    //let hash = comm::auth::signup("ductape".to_owned(), "password".to_owned())?;
    // println!("Login: {}, time: {}", &hash, now.elapsed().as_millis());
    // println!("pending...");
    // let now = Instant::now();
    //println!("Verify: {}, time: {}", login_test("username".to_owned(), "password".to_owned(), &hash)?, now.elapsed().as_millis());


    let mut data: HashMap<&str, serde_json::Value> = HashMap::new();
    data.insert("conf", engine_json);
    data.insert("pokemon", pokedex_json);
    data.insert("moves", movedex_json);

    let engine = init_engine(data, words_file);
    //println!("Types: {:#?}", &typedex_vec.into_iter().find(|x| x.index == 23));
    assert_eq!(engine.pokedex.len(), 151, "Pokedex length should be {} but {} was found", 151, engine.pokedex.len());

    println!("{}", engine.pokedex.get(&(rng.gen_range(1..=engine.pokedex.len()) as u8)).unwrap().debug_graphic());

    let server_configs = create_server_config(Some(8080))?;
    println!("{:#?}", server_configs);

    let addr = format!("{}:{}", server_configs.ip, server_configs.port);
    let state = server_configs.peers;

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    //Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr, engine.clone()));
    }

    Ok(())

}
